use bevy::prelude::*;
use hotkey_plugin::hotkey_listener::HotkeyListener;
use hotkey_plugin::hotkey_plugin::HotkeyPlugin;
use hotkey_plugin::hotkey_states::HotkeyStates;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use ui::change_button_text_system;
use ui::setup_ui;
use ui::ApplySettingsButton;
use ui::HotkeyButton;

mod ui;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Action {
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
            hotkey_listener.set_currently_listening(&hotkey_name.action, hotkey_name.num)
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
