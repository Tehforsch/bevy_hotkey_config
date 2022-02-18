# bevy_hotkey_config

A hotkey abstraction for bevy. Allows binding arbitrary keyboard + mouse hotkeys (including modifiers) to actions specified in an enum. The hotkey configuration can be modified at app runtime and saved/loaded from a file, allowing the user to set hotkeys for themselves.

Anything can be used as the action map, provided it implements `Serialize`, `Deserialize`, `Clone`, `PartialEq`, `Eq` and `Hash`

Whether a hotkey is used can be used from bevy systems by requesting `Res<HotkeyStates<...>>` and calling any of the methods `pressed`, `just_pressed`, ... just as in bevy. In addition, a `repeated` method is provided which triggers in a given, configurable frequency, as long as a hotkey is held down.

Modifiers are checked consistently, meaning that pressing `Ctrl + W` will not trigger any hotkey bound to `W`.

# Basic usage
See `examples/from_file.rs`
```
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
enum GameAction {
    WalkLeft,
    WalkRight,
    Jump,
    Duck,
}

fn main() {
    let mut app = App::new();

    let config = serde_json::from_reader(std::fs::File::open("assets/hotkeys.json").unwrap()).unwrap();

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
```

# Configuring hotkeys
As an example for how to change hotkeys during app run, see `examples/settings`.
