use bevy::prelude::App;
use bevy::prelude::Res;
use bevy::DefaultPlugins;
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

fn main() {
    let mut app = App::new();

    let config = serde_json::from_str(include_str!("assets/settings.json")).unwrap();

    app.add_plugins(DefaultPlugins)
        .add_plugin(HotkeyPlugin::<Action>::new(config))
        .add_system(input_system);

    app.run();
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
