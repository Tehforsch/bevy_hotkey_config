pub mod action;
mod config;
mod hotkey;
pub mod hotkey_config;
pub mod hotkey_listener;
pub mod hotkey_plugin;
mod hotkey_state;
pub mod hotkey_states;
mod key_repeat_state;
pub mod modifier;
mod mouse_wheel_action;
mod window_focus_state;

use bevy::input::mouse::MouseWheel;
use bevy::input::Input;
use bevy::prelude::KeyCode;
use bevy::prelude::MouseButton;
use hotkey::Hotkey;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Hotkeys(Vec<Hotkey>);

impl Hotkeys {
    pub(crate) fn new(hotkeys: Vec<Hotkey>) -> Self {
        Self(hotkeys)
    }

    pub(crate) fn push(&mut self, hotkey: Hotkey) {
        self.0.push(hotkey)
    }

    pub(crate) fn just_pressed(
        &self,
        keyboard_input: &Input<KeyCode>,
        mouse_input: &Input<MouseButton>,
        mouse_wheel_events: &[&MouseWheel],
    ) -> bool {
        self.0
            .iter()
            .any(|hotkey| hotkey.just_pressed(keyboard_input, mouse_input, mouse_wheel_events))
    }

    pub(crate) fn pressed(
        &self,
        keyboard_input: &Input<KeyCode>,
        mouse_input: &Input<MouseButton>,
    ) -> bool {
        self.0
            .iter()
            .any(|hotkey| hotkey.pressed(keyboard_input, mouse_input))
    }

    pub(crate) fn try_remove_hotkey(&mut self, num: usize) {
        if self.0.len() > num {
            self.0.remove(num);
        }
    }

    pub(crate) fn change_hotkey(&mut self, num: usize, hotkey: Hotkey) {
        if self.0.len() <= num {
            self.0.push(hotkey);
        } else {
            self.0[num] = hotkey;
        }
    }

    pub fn iter(&self) -> std::slice::Iter<Hotkey> {
        self.0.iter()
    }

    pub fn get(&self, num: usize) -> Option<&Hotkey> {
        self.0.get(num)
    }
}
