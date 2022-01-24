use bevy::prelude::App;
use bevy::prelude::Res;
use bevy::DefaultPlugins;
use hotkey_plugin::hotkey_plugin::HotkeyPlugin;
use hotkey_plugin::hotkey_states::HotkeyStates;
use serde::Deserialize;
use serde::Serialize;
use serde_json;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
enum GameAction {
    WalkLeft,
    WalkRight,
    Jump,
    Duck,
}

fn main() {
    let mut app = App::new();

    let config = serde_json::from_str(include_str!("../assets/settings.json")).unwrap();

    app.add_plugins(DefaultPlugins)
        .add_plugin(HotkeyPlugin::<GameAction>::new(config))
        .add_system(input_system);

    app.run();
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
