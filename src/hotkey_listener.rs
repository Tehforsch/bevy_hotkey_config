use core::hash::Hash;

use bevy::input::Input;
use bevy::prelude::Component;
use bevy::prelude::KeyCode;
use bevy::prelude::Res;
use bevy::prelude::ResMut;

use crate::action::Action;
use crate::hotkey::Hotkey;
use crate::hotkey_config::HotkeyConfig;
use crate::hotkey_states::HotkeyStates;
use crate::modifier::Modifier;
use crate::modifier::AVAILABLE_MODIFIERS;

#[derive(Component)]
pub struct HotkeyListener<T> {
    currently_listening: Option<(T, usize)>,
    cancel_action: Action,
    remove_action: Action,
    should_apply_settings: bool,
}

impl<T> HotkeyListener<T> {
    pub fn new(cancel_action: Action, remove_action: Action) -> Self {
        Self {
            currently_listening: None,
            cancel_action,
            remove_action,
            should_apply_settings: false,
        }
    }
}

impl<T> HotkeyListener<T>
where
    T: Clone + Sync + Send + 'static + PartialEq + Eq + Hash,
{
    pub fn set_currently_listening(&mut self, currently_listening: &T, num: usize) {
        self.currently_listening = Some((currently_listening.clone(), num))
    }

    pub fn clear_currently_listening(&mut self) {
        self.currently_listening = None
    }

    pub fn apply_settings(&mut self) {
        self.should_apply_settings = true
    }

    pub(crate) fn listen_system(
        mut listener: ResMut<Self>,
        mut settings_hotkeys: ResMut<HotkeyConfig<T>>,
        input: Res<Input<KeyCode>>,
    ) {
        if listener.currently_listening.is_none() {
            return;
        }
        let (modifiers_pressed, other_keys_pressed): (Vec<KeyCode>, Vec<KeyCode>) = input
            .get_pressed()
            .partition(|key| AVAILABLE_MODIFIERS.iter().any(|modifier| modifier.is(key)));
        let modifiers_pressed = modifiers_pressed
            .into_iter()
            .filter_map(|key_code| Modifier::from_key_code(&key_code))
            .collect();
        if other_keys_pressed.len() == 1 {
            let pressed_key = other_keys_pressed.first().unwrap();
            listener.assign(
                &mut settings_hotkeys,
                modifiers_pressed,
                Action::Key(pressed_key.clone()),
            );
        }
    }

    pub(crate) fn apply_hotkey_system(
        listener: ResMut<Self>,
        config: Res<HotkeyConfig<T>>,
        mut live_hotkeys: ResMut<HotkeyStates<T>>,
    ) {
        if listener.should_apply_settings {
            live_hotkeys.config = config.clone();
        }
    }

    fn assign(&mut self, config: &mut HotkeyConfig<T>, modifiers: Vec<Modifier>, action: Action) {
        // We know that we are listening for some hotkey, so we can unwrap.
        // This also resets the listening state, so after this function call
        // we will not listen anymore.
        let (hotkey, num) = self.currently_listening.take().unwrap();
        let current_hotkeys = config.map.get_mut(&hotkey);
        if action == self.cancel_action {
            return;
        }
        if let Some(current_hotkeys) = current_hotkeys {
            if action == self.remove_action {
                current_hotkeys.try_remove_hotkey(num);
                return;
            }
            let new_hotkey = Hotkey {
                key: action,
                modifiers,
            };
            current_hotkeys.change_hotkey(num, new_hotkey);
        }
    }
}
