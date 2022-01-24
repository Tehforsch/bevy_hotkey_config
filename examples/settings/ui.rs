use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use hotkey_plugin::hotkey_config::HotkeyConfig;

use crate::Action;

pub const NUM_HOTKEYS_TO_SHOW: usize = 3;

#[derive(Component)]
pub struct HotkeyButton {
    pub action: Action,
    pub num: usize,
}

#[derive(Component)]
pub struct ApplySettingsButton;

pub fn change_button_text_system(
    button_query: Query<(&HotkeyButton, &Children), With<Button>>,
    mut text_query: Query<&mut Text>,
    config: Res<HotkeyConfig<Action>>,
) {
    if config.is_changed() {
        for (button, children) in button_query.iter() {
            let mut text = text_query.get_mut(children[0]).unwrap();
            if let Some(hotkeys) = config.get(&button.action) {
                text.sections[0].value = match hotkeys.get(button.num) {
                    None => "+".into(),
                    Some(hotkey) => hotkey.to_string(),
                }
            }
        }
    }
}

fn spawn_button<'w, 's, 'a, 'b>(
    parent: &'a mut ChildBuilder<'w, 's, 'b>,
    text: &str,
    font: &Handle<Font>,
) -> EntityCommands<'w, 's, 'a> {
    let mut entity = parent.spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(200.0), Val::Px(65.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: Color::rgb(0.25, 0.25, 0.25).into(),
        ..Default::default()
    });
    entity.with_children(|parent| {
        spawn_text(parent, text, font, None);
    });
    entity
}

fn spawn_text<'w, 's, 'a, 'b>(
    parent: &'a mut ChildBuilder<'w, 's, 'b>,
    text: &str,
    font: &Handle<Font>,
    size: Option<Size<Val>>,
) -> EntityCommands<'w, 's, 'a> {
    let size = size.unwrap_or(Size::default());
    parent.spawn_bundle(TextBundle {
        style: Style {
            size,
            ..Default::default()
        },
        text: Text::with_section(
            text,
            TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
            Default::default(),
        ),
        ..Default::default()
    })
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            for action in [
                Action::WalkLeft,
                Action::WalkRight,
                Action::Jump,
                Action::Duck,
            ] {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        color: Color::NONE.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        spawn_text(
                            parent,
                            action.get_name(),
                            &font,
                            Some(Size::new(Val::Px(200.0), Val::Px(65.0))),
                        );
                        for num in 0..NUM_HOTKEYS_TO_SHOW {
                            // Spawn with empty text, this is going to get set in change_button_text_system immediately anyways
                            spawn_button(parent, "".into(), &font).insert(HotkeyButton {
                                action: action.clone(),
                                num,
                            });
                        }
                    });
            }
            spawn_button(parent, "Apply".into(), &font).insert(ApplySettingsButton);
        });
}
