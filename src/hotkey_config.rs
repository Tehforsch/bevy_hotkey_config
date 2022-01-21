use std::collections::HashMap;
use std::hash::Hash;

use bevy::prelude::KeyCode;
use serde::Deserialize;
use serde::Serialize;

use super::Hotkeys;
use crate::action::Action;
use crate::config;
use crate::hotkey::Hotkey;
use crate::modifier::Modifier;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HotkeyConfig<T: Hash + Eq + Clone> {
    map: HashMap<T, Hotkeys>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyRepeatSettings {
    pub initial_delay: f32,
    pub delay: f32,
}

impl Default for KeyRepeatSettings {
    fn default() -> Self {
        KeyRepeatSettings {
            initial_delay: config::DEFAULT_REPEAT_INITIAL_DELAY,
            delay: config::DEFAULT_REPEAT_DELAY,
        }
    }
}

impl<T: Eq + Hash + Clone> HotkeyConfig<T> {
    pub fn empty() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert_normal(&mut self, name: T, key_code: KeyCode) {
        let hotkey = Hotkey {
            key: Action::Key(key_code),
            modifiers: vec![],
        };
        self.insert(name, hotkey)
    }

    pub fn insert_with_modifiers(&mut self, name: T, key_code: KeyCode, modifiers: &[Modifier]) {
        let hotkey = Hotkey {
            key: Action::Key(key_code),
            modifiers: modifiers.into_iter().cloned().collect(),
        };
        self.insert(name, hotkey)
    }

    fn insert(&mut self, name: T, hotkey: Hotkey) {
        match self.map.get_mut(&name) {
            Some(hotkeys) => hotkeys.push(hotkey),
            None => {
                self.map.insert(name, Hotkeys::new(vec![hotkey]));
            }
        }
    }

    pub fn update_from(&mut self, config: &HotkeyConfig<T>) {
        for (key, value) in config.map.iter() {
            self.map.entry(key.clone()).or_insert(value.clone());
        }
    }

    pub(crate) fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a T, &'a Hotkeys)> {
        self.map.iter()
    }
}
