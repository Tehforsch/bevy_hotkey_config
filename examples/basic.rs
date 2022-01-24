use bevy::prelude::App;
use bevy::prelude::KeyCode;
use bevy::prelude::Res;
use bevy::DefaultPlugins;
use hotkey_plugin::hotkey_config::HotkeyConfig;
use hotkey_plugin::hotkey_plugin::HotkeyPlugin;
use hotkey_plugin::hotkey_states::HotkeyStates;
use hotkey_plugin::modifier::Modifier;

#[derive(Clone, PartialEq, Eq, Hash)]
enum GameAction {
    WalkLeft,
    WalkRight,
    Jump,
    Duck,
}

fn main() {
    let mut app = App::new();
    let mut config = HotkeyConfig::<GameAction>::empty();
    config.insert_normal(GameAction::WalkLeft, KeyCode::A);
    config.insert_normal(GameAction::WalkLeft, KeyCode::Left);
    config.insert_with_modifiers(GameAction::WalkLeft, KeyCode::A, &[Modifier::Control]);
    config.insert_normal(GameAction::WalkRight, KeyCode::D);
    config.insert_normal(GameAction::WalkRight, KeyCode::Right);
    config.insert_with_modifiers(GameAction::WalkRight, KeyCode::D, &[Modifier::Control]);
    config.insert_normal(GameAction::Jump, KeyCode::W);
    config.insert_normal(GameAction::Jump, KeyCode::Up);
    config.insert_normal(GameAction::Duck, KeyCode::S);
    config.insert_normal(GameAction::Duck, KeyCode::Down);

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
