use std::hash::Hash;

use bevy::prelude::*;

use super::hotkey_states::reset_input_system;
use super::hotkey_states::set_hotkey_states_from_input_system;
use super::hotkey_states::HotkeyStates;
use super::window_focus_state::WindowFocusState;
use crate::hotkey_config::HotkeyConfig;
use crate::hotkey_config::KeyRepeatSettings;

#[derive(PartialEq, Eq, Hash, Clone, Debug, SystemLabel)]
enum HotkeySystems {
    SetHotkeyStates,
    InputReset,
}

pub struct HotkeyPlugin<T: Eq + Hash + Clone> {
    config: HotkeyConfig<T>,
    key_repeat: KeyRepeatSettings,
}

impl<T: Eq + Hash + Clone> HotkeyPlugin<T> {
    pub fn new(config: HotkeyConfig<T>) -> Self {
        Self {
            config,
            key_repeat: KeyRepeatSettings::default(),
        }
    }
}

impl<T: Sync + Send + 'static + Eq + Hash + Clone> Plugin for HotkeyPlugin<T> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(HotkeyStates::from_settings(
            self.config.clone(),
            self.key_repeat.clone(),
        ))
        .init_resource::<WindowFocusState>()
        .add_system(set_hotkey_states_from_input_system::<T>.label(HotkeySystems::SetHotkeyStates))
        .add_system(
            reset_input_system
                .label(HotkeySystems::InputReset)
                .after(HotkeySystems::SetHotkeyStates),
        );
    }
}

pub fn hotkey_flush_system<T: Sync + Send + 'static + Eq + Hash + Clone>(
    mut hotkeys: ResMut<HotkeyStates<T>>,
) {
    hotkeys.flush();
}
