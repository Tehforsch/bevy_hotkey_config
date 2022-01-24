use bevy::prelude::*;
use hotkey_plugin::action::Action;
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
pub enum GameAction {
    WalkLeft,
    WalkRight,
    Jump,
    Duck,
}

impl GameAction {
    fn get_name(&self) -> &str {
        match self {
            GameAction::WalkLeft => "Walk left",
            GameAction::WalkRight => "Walk right",
            GameAction::Jump => "Jump",
            GameAction::Duck => "Duck",
        }
    }
}

fn main() {
    let config = serde_json::from_str(include_str!("../../assets/settings.json")).unwrap();
    App::new()
        .add_plugin(
            HotkeyPlugin::<GameAction>::new(config)
                .allow_modification(Action::Key(KeyCode::Escape), Action::Key(KeyCode::Back)),
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
    mut hotkey_listener: ResMut<HotkeyListener<GameAction>>,
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
    mut hotkey_listener: ResMut<HotkeyListener<GameAction>>,
) {
    if let Some(Interaction::Clicked) = interaction_query.iter().next() {
        hotkey_listener.apply_settings()
    }
}

fn input_system(hotkeys: Res<HotkeyStates<GameAction>>) {
    if hotkeys.repeated(GameAction::WalkLeft) {
        println!("Walking to the left");
    }
    if hotkeys.repeated(GameAction::WalkRight) {
        println!("Walking to the right");
    }
    if hotkeys.just_pressed(GameAction::Jump) {
        println!("Jumping");
    }
    if hotkeys.pressed(GameAction::Duck) {
        println!("Ducked");
    }
    if hotkeys.just_released(GameAction::Duck) {
        println!("Stopped ducking");
    }
}
