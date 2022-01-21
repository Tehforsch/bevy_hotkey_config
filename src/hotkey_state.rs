use bevy::core::Time;
use bevy::input::mouse::MouseWheel;
use bevy::input::Input;
use bevy::prelude::KeyCode;
use bevy::prelude::MouseButton;

use super::key_repeat_state::KeyRepeatState;
use super::Hotkeys;
use crate::hotkey_config::KeyRepeatSettings;

#[derive(Clone)]
pub(crate) struct HotkeyState {
    pub just_pressed: bool,
    pub pressed: bool,
    pub just_released: bool,
    pub repeat_state: KeyRepeatState,
    pub repeated: bool,
}

impl HotkeyState {
    pub(crate) fn from_settings(key_repeat_settings: &KeyRepeatSettings) -> Self {
        HotkeyState {
            just_pressed: false,
            pressed: false,
            just_released: false,
            repeat_state: KeyRepeatState::from_settings(key_repeat_settings),
            repeated: false,
        }
    }

    pub(crate) fn update(
        &mut self,
        hotkey: &Hotkeys,
        keyboard_input: &Input<KeyCode>,
        mouse_input: &Input<MouseButton>,
        mouse_wheel_events: &[&MouseWheel],
        time: &Time,
    ) {
        self.just_pressed = hotkey.just_pressed(keyboard_input, mouse_input, mouse_wheel_events);
        let previously_pressed = self.pressed;
        self.pressed = hotkey.pressed(keyboard_input, mouse_input);
        self.just_released = previously_pressed && !self.pressed;
        self.repeat_state.tick(time, self.pressed);
        self.repeated = self.repeat_state.key_repeated() || self.just_pressed;
    }

    pub(crate) fn reset(&mut self) {
        self.just_pressed = false;
        self.pressed = false;
        self.just_released = false;
        self.repeated = false;
        self.repeat_state.reset();
    }
}
