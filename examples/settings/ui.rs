use bevy::prelude::*;
use hotkey_plugin::hotkey_config::HotkeyConfig;
use hotkey_plugin::hotkey_states::HotkeyStates;
use hotkey_plugin::Hotkeys;

use crate::Action;

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
        for (hotkey, children) in button_query.iter() {
            let mut text = text_query.get_mut(children[0]).unwrap();
            if let Some(hotkeys) = config.get(&hotkey.action) {
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

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    hotkeys: Res<HotkeyStates<Action>>,
) {
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
                Action::Duck,
                Action::Jump,
                Action::WalkLeft,
                Action::WalkRight,
            ] {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::FlexStart,
                            ..Default::default()
                        },
                        color: Color::NONE.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            style: Style {
                                size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                                ..Default::default()
                            },
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
                            .insert(HotkeyButton {
                                action: action.clone(),
                                num: 0,
                            })
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
        });
}
