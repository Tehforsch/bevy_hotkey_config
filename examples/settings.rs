use bevy::prelude::*;
use hotkey_plugin::hotkey_plugin::HotkeyPlugin;
use hotkey_plugin::hotkey_states::HotkeyStates;
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

struct SelectedHotkey(Option<Action>);

#[derive(Component)]
struct HotkeySelector(Action);

fn main() {
    let config = serde_json::from_str(include_str!("../assets/settings.json")).unwrap();
    App::new()
        .add_plugin(HotkeyPlugin::<Action>::new(config))
        .add_plugins(DefaultPlugins)
        .insert_resource(SelectedHotkey(None))
        .add_startup_system(setup_ui)
        .add_system(change_hotkey_system)
        .run();
}

fn change_hotkey_system(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<Children>),
    >,
    currently_selected_hotkey: Res<SelectedHotkey>,
) {
    for interaction in interaction_query.iter_mut() {
        if let Interaction::Clicked = interaction {}
    }
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
                    flex_direction: FlexDirection::ColumnReverse,
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
                    .insert(HotkeySelector(action.clone()))
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                hotkeys
                                    .get(&action)
                                    .unwrap()
                                    .iter()
                                    .map(|hotkey| hotkey.to_string())
                                    .collect::<Vec<String>>()
                                    .join(","),
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
}