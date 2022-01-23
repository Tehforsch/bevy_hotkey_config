use std::collections::HashMap;
use std::hash::Hash;

use bevy::core::Time;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::EventReader;
use bevy::prelude::Input;
use bevy::prelude::KeyCode;
use bevy::prelude::MouseButton;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::window::Windows;

use super::hotkey_config::HotkeyConfig;
use super::hotkey_state::HotkeyState;
use super::window_focus_state::WindowFocusState;
use crate::hotkey_config::KeyRepeatSettings;
use crate::Hotkeys;

pub struct HotkeyStates<T: Eq + Hash + Clone> {
    config: HotkeyConfig<T>,
    key_repeat: KeyRepeatSettings,
    states: HashMap<T, HotkeyState>,
}

impl<T: Eq + Hash + Clone> HotkeyStates<T> {
    pub fn from_settings(config: HotkeyConfig<T>, key_repeat: KeyRepeatSettings) -> Self {
        Self {
            states: HashMap::new(),
            key_repeat,
            config,
        }
    }

    pub fn iter_just_pressed(&self) -> impl Iterator<Item = &T> + '_ {
        Box::new(
            self.states
                .iter()
                .filter(|(_, state)| state.just_pressed)
                .map(|(name, _)| name),
        )
    }

    pub fn flush(&mut self) {
        for (_, state) in self.states.iter_mut() {
            state.reset();
        }
    }

    pub fn just_pressed(&self, name: T) -> bool {
        self.states
            .get(&name)
            .map(|state| state.just_pressed)
            .unwrap_or(false)
    }

    pub fn just_released(&self, name: T) -> bool {
        self.states
            .get(&name)
            .map(|state| state.just_released)
            .unwrap_or(false)
    }

    pub fn pressed(&self, name: T) -> bool {
        self.states
            .get(&name)
            .map(|state| state.pressed)
            .unwrap_or(false)
    }

    pub fn repeated(&self, name: T) -> bool {
        self.states
            .get(&name)
            .map(|state| state.repeated)
            .unwrap_or(false)
    }

    pub fn update(
        &mut self,
        keyboard_input: &Input<KeyCode>,
        mouse_input: &Input<MouseButton>,
        mouse_wheel_events: &[&MouseWheel],
        time: &Time,
    ) {
        for (name, hotkey) in self.config.iter() {
            let state = self.states.get_mut(name);
            match state {
                None => {
                    self.states
                        .insert(name.clone(), HotkeyState::from_settings(&self.key_repeat));
                }
                Some(state) => state.update(
                    hotkey,
                    keyboard_input,
                    mouse_input,
                    mouse_wheel_events,
                    time,
                ),
            }
        }
    }

    pub fn get(&self, name: &T) -> Option<&Hotkeys> {
        self.config.map.get(name)
    }

    pub(crate) fn change_config_for(&mut self, name: T, hotkeys: Hotkeys) {
        self.config.map.insert(name, hotkeys);
    }

    #[cfg(test)]
    pub fn just_press_hotkey(&mut self, name: T) {
        self.states
            .insert(name.clone(), HotkeyState::from_settings(&self.key_repeat));
        let state = self.states.get_mut(&name).unwrap();
        state.pressed = true;
        state.just_pressed = true;
    }

    #[cfg(test)]
    pub fn press_hotkey(&mut self, name: T) {
        self.states
            .insert(name.clone(), HotkeyState::from_settings(&self.key_repeat));
        let state = self.states.get_mut(&name).unwrap();
        state.pressed = true;
    }

    #[cfg(test)]
    pub fn release_hotkey(&mut self, name: T) {
        self.states
            .insert(name.clone(), HotkeyState::from_settings(&self.key_repeat));
        let state = self.states.get_mut(&name).unwrap();
        state.just_released = true;
    }
}

pub(super) fn set_hotkey_states_from_input_system<T: Sync + Send + 'static + Eq + Hash + Clone>(
    mut hotkey_states: ResMut<HotkeyStates<T>>,
    mut window_focus_state: ResMut<WindowFocusState>,
    windows: Res<Windows>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    time: Res<Time>,
) {
    let window = windows.iter().next().unwrap();
    if window_focus_state.interaction_allowed(window, &keyboard_input, &mouse_input) {
        let mouse_wheel_events: Vec<_> = mouse_wheel_events.iter().collect();
        hotkey_states.update(&keyboard_input, &mouse_input, &mouse_wheel_events, &time);
    } else {
        // The following is a (hopefully) temporary fix for a bug in bevy (or winit).
        // This bug will keep any key which was pressed in the moment that the window
        // got back into focus pressed until the key is pressed again.
        // The next line forcefully un-presses any pressed key to prevent
        // this from happening.
        // (input.update() is not enough here, since that only clears just_pressed.)
        let pressed = keyboard_input.get_pressed().copied().collect::<Vec<_>>();
        for key in pressed.into_iter() {
            keyboard_input.release(key);
        }
    }
}

pub(super) fn reset_input_system(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut mouse_input: ResMut<Input<MouseButton>>,
) {
    keyboard_input.clear();
    mouse_input.clear();
}
