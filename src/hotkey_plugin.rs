use std::hash::Hash;

use bevy::prelude::KeyCode;
use bevy::prelude::ParallelSystemDescriptorCoercion;
use bevy::prelude::Plugin;
use bevy::prelude::ResMut;
use bevy::prelude::SystemLabel;

use super::hotkey_states::reset_input_system;
use super::hotkey_states::set_hotkey_states_from_input_system;
use super::hotkey_states::HotkeyStates;
use super::window_focus_state::WindowFocusState;
use crate::hotkey_config::HotkeyConfig;
use crate::hotkey_config::KeyRepeatSettings;
use crate::hotkey_listener::HotkeyListener;

#[derive(PartialEq, Eq, Hash, Clone, Debug, SystemLabel)]
enum HotkeySystems {
    SetHotkeyStates,
    InputReset,
}

pub struct HotkeyPlugin<T: Eq + Hash + Clone> {
    config: HotkeyConfig<T>,
    key_repeat: KeyRepeatSettings,
    listener_settings: Option<(KeyCode, KeyCode)>,
}

impl<T: Eq + Hash + Clone> HotkeyPlugin<T> {
    pub fn new(config: HotkeyConfig<T>) -> Self {
        Self {
            config,
            key_repeat: KeyRepeatSettings::default(),
            listener_settings: None,
        }
    }

    pub fn allow_modification(mut self, cancel_key: KeyCode, remove_key: KeyCode) -> Self {
        self.listener_settings = Some((cancel_key, remove_key));
        self
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
        if let Some((cancel_key, remove_key)) = self.listener_settings {
            app.insert_resource(HotkeyListener::<T>::new(cancel_key, remove_key))
                .insert_resource(self.config.clone())
                .add_system(HotkeyListener::<T>::apply_hotkey_system)
                .add_system(HotkeyListener::<T>::listen_system);
        }
    }
}

pub fn hotkey_flush_system<T: Sync + Send + 'static + Eq + Hash + Clone>(
    mut hotkeys: ResMut<HotkeyStates<T>>,
) {
    hotkeys.flush();
}
