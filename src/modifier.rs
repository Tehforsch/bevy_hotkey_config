use bevy::prelude::KeyCode;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Modifier {
    Control,
    Shift,
    Alt,
}

impl Modifier {
    pub fn is(&self, key_code: &KeyCode) -> bool {
        self.get_key_codes().contains(key_code)
    }

    pub fn get_key_codes(&self) -> Vec<KeyCode> {
        match self {
            Modifier::Control => vec![KeyCode::LControl, KeyCode::RControl],
            Modifier::Shift => vec![KeyCode::LShift, KeyCode::RShift],
            Modifier::Alt => vec![KeyCode::LAlt, KeyCode::RAlt],
        }
    }

    pub fn from_key_code(key_code: &KeyCode) -> Option<Self> {
        match key_code {
            KeyCode::LControl => Some(Self::Control),
            KeyCode::RControl => Some(Self::Control),
            KeyCode::LShift => Some(Self::Shift),
            KeyCode::RShift => Some(Self::Shift),
            KeyCode::LAlt => Some(Self::Alt),
            KeyCode::RAlt => Some(Self::Alt),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Modifier::Control => "ctrl",
            Modifier::Shift => "shift",
            Modifier::Alt => "alt",
        }
    }
}

pub const AVAILABLE_MODIFIERS: &[Modifier] = &[Modifier::Control, Modifier::Shift, Modifier::Alt];
