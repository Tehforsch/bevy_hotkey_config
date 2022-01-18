use std::collections::HashMap;
use std::hash::Hash;

use serde::Deserialize;
use serde::Serialize;

use super::Hotkeys;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HotkeyConfig<T: Hash + Eq + Clone> {
    pub map: HashMap<T, Hotkeys>,
    pub key_repeat: KeyRepeat,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyRepeat {
    pub initial_delay: f32,
    pub delay: f32,
}

impl Default for KeyRepeat {
    fn default() -> Self {
        KeyRepeat {
            initial_delay: 0.3,
            delay: 0.15,
        }
    }
}

impl<T: Eq + Hash + Clone> HotkeyConfig<T> {
    pub fn get(&self, hotkey: &T) -> Hotkeys {
        self.map
            .get(hotkey)
            .cloned()
            .unwrap_or_else(|| Hotkeys::new(vec![]))
    }

    pub fn update_from(&mut self, config: &HotkeyConfig<T>) {
        for (key, value) in config.map.iter() {
            self.map.entry(key.clone()).or_insert(value.clone());
        }
    }
}
