use core::hash::Hash;

use bevy::input::Input;
use bevy::prelude::Component;
use bevy::prelude::KeyCode;
use bevy::prelude::Res;
use bevy::prelude::ResMut;

use crate::action::Action;
use crate::hotkey::Hotkey;
use crate::hotkey_states::HotkeyStates;
use crate::modifier::Modifier;
use crate::modifier::AVAILABLE_MODIFIERS;
use crate::Hotkeys;

#[derive(Component)]
pub struct HotkeyListener<T> {
    currently_listening: Option<(T, usize)>,
    remove_hotkey: KeyCode,
    cancel_hotkey: KeyCode,
}

impl<T> HotkeyListener<T> {
    pub fn new(cancel_hotkey: KeyCode, remove_hotkey: KeyCode) -> Self {
        Self {
            currently_listening: None,
            remove_hotkey,
            cancel_hotkey,
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

    pub fn listen_system(
        mut listener: ResMut<Self>,
        mut hotkey_states: ResMut<HotkeyStates<T>>,
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
                &mut hotkey_states,
                modifiers_pressed,
                Action::Key(pressed_key.clone()),
            );
        }
    }

    fn assign(
        &mut self,
        hotkey_states: &mut HotkeyStates<T>,
        modifiers: Vec<Modifier>,
        action: Action,
    ) {
        // We know that we are listening for some hotkey, so we can unwrap.
        // This also resets the listening state, so after this function call
        // we will not listen anymore.
        let (hotkey, num) = self.currently_listening.take().unwrap();
        let mut current_hotkeys = hotkey_states
            .get(&hotkey)
            .cloned()
            .unwrap_or(Hotkeys::new(vec![]));
        if let Action::Key(key) = action {
            if key == self.remove_hotkey {
                current_hotkeys.try_remove_hotkey(num);
                return;
            }
            if key == self.cancel_hotkey {
                return;
            }
        }
        let new_hotkey = Hotkey {
            key: action,
            modifiers,
        };
        current_hotkeys.change_hotkey(num, new_hotkey);
        hotkey_states.change_config_for(hotkey.clone(), current_hotkeys);
    }
}
