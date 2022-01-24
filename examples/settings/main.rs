use bevy::prelude::*;
use hotkey_plugin::hotkey_config::HotkeyConfig;
use hotkey_plugin::hotkey_listener::HotkeyListener;
use hotkey_plugin::hotkey_plugin::HotkeyPlugin;
use hotkey_plugin::hotkey_states::HotkeyStates;
use hotkey_plugin::Hotkeys;
use serde::Deserialize;
use serde::Serialize;
use serde_json;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
enum Action {
    WalkLeft,
    WalkRight,
    Jump,
    Duck,
}

impl Action {
    fn get_name(&self) -> &str {
        match self {
            Action::WalkLeft => "Walk left",
            Action::WalkRight => "Walk right",
            Action::Jump => "Jump",
            Action::Duck => "Duck",
        }
    }
}

#[derive(Component)]
struct HotkeyButton(Action);

#[derive(Component)]
struct ApplySettingsButton;

fn main() {
    let config = serde_json::from_str(include_str!("../../assets/settings.json")).unwrap();
    App::new()
        .add_plugin(
            HotkeyPlugin::<Action>::new(config).allow_modification(KeyCode::Escape, KeyCode::Back),
        )
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_ui)
        .add_system(select_hotkeys_system)
        .add_system(apply_hotkeys_system)
        .add_system(change_button_text_system)
        .add_system(input_system)
        .run();
}

fn select_hotkeys_system(
    mut interaction_query: Query<
        (&Interaction, &HotkeyButton),
        (Changed<Interaction>, With<Button>, With<Children>),
    >,
    mut hotkey_listener: ResMut<HotkeyListener<Action>>,
) {
    for (interaction, hotkey_name) in interaction_query.iter_mut() {
        if let Interaction::Clicked = interaction {
            hotkey_listener.set_currently_listening(&hotkey_name.0, 0)
        }
    }
}

fn apply_hotkeys_system(
    interaction_query: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<Button>,
            With<ApplySettingsButton>,
        ),
    >,
    mut hotkey_listener: ResMut<HotkeyListener<Action>>,
) {
    if let Some(Interaction::Clicked) = interaction_query.iter().next() {
        hotkey_listener.apply_settings()
    }
}

fn change_button_text_system(
    interaction_query: Query<(&Interaction, &HotkeyButton, &Children), (With<Button>,)>,
    mut text_query: Query<&mut Text>,
    config: Res<HotkeyConfig<Action>>,
) {
    if config.is_changed() {
        for (interaction, hotkey, children) in interaction_query.iter() {
            let mut text = text_query.get_mut(children[0]).unwrap();
            if let Some(hotkeys) = config.get(&hotkey.0) {
                text.sections[0].value = hotkeys_to_string(hotkeys);
            }
        }
    }
}

fn hotkeys_to_string(hotkeys: &Hotkeys) -> String {
    hotkeys
        .iter()
        .map(|hotkey| hotkey.to_string())
        .collect::<Vec<String>>()
        .join(",")
        .into()
}

fn input_system(hotkeys: Res<HotkeyStates<Action>>) {
    if hotkeys.repeated(Action::WalkLeft) {
        println!("Walking to the left");
    }
    if hotkeys.repeated(Action::WalkRight) {
        println!("Walking to the right");
    }
    if hotkeys.just_pressed(Action::Jump) {
        println!("Jumping");
    }
    if hotkeys.pressed(Action::Duck) {
        println!("Ducked");
    }
    if hotkeys.just_released(Action::Duck) {
        println!("Stopped ducking");
    }
}

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    hotkeys: Res<HotkeyStates<Action>>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    for action in [
        Action::Duck,
        Action::Jump,
        Action::WalkLeft,
        Action::WalkRight,
    ] {
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                    margin: Rect::all(Val::Auto),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        action.get_name(),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        color: Color::rgb(0.25, 0.25, 0.25).into(),
                        ..Default::default()
                    })
                    .insert(HotkeyButton(action.clone()))
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                hotkeys_to_string(hotkeys.get(&action).unwrap()),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 30.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        });
                    });
            });
    }
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgb(0.25, 0.25, 0.25).into(),
            ..Default::default()
        })
        .insert(ApplySettingsButton)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Apply",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}
