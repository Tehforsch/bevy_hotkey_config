use bevy::prelude::App;
use bevy::prelude::KeyCode;
use bevy::prelude::Res;
use bevy::DefaultPlugins;
use hotkey_plugin::hotkey_config::HotkeyConfig;
use hotkey_plugin::hotkey_plugin::HotkeyPlugin;
use hotkey_plugin::hotkey_states::HotkeyStates;
use hotkey_plugin::modifier::Modifier;

#[derive(Clone, PartialEq, Eq, Hash)]
enum Action {
    WalkLeft,
    WalkRight,
    Jump,
    Duck,
}

fn main() {
    let mut app = App::new();
    let mut config = HotkeyConfig::<Action>::empty();
    config.insert_normal(Action::WalkLeft, KeyCode::A);
    config.insert_normal(Action::WalkLeft, KeyCode::Left);
    config.insert_with_modifiers(Action::WalkLeft, KeyCode::A, &[Modifier::Control]);
    config.insert_normal(Action::WalkRight, KeyCode::D);
    config.insert_normal(Action::WalkRight, KeyCode::Right);
    config.insert_with_modifiers(Action::WalkRight, KeyCode::D, &[Modifier::Control]);
    config.insert_normal(Action::Jump, KeyCode::W);
    config.insert_normal(Action::Jump, KeyCode::Up);
    config.insert_normal(Action::Duck, KeyCode::S);
    config.insert_normal(Action::Duck, KeyCode::Down);

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
