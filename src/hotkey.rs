use std::fmt::Display;
use std::iter;

use bevy::input::mouse::MouseWheel;
use bevy::input::Input;
use bevy::prelude::KeyCode;
use bevy::prelude::MouseButton;
use serde::Deserialize;
use serde::Serialize;

use super::action::Action;
use super::mouse_wheel_action::MouseWheelAction;

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

    pub fn from(key_code: &KeyCode) -> Option<Self> {
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
}

pub const AVAILABLE_MODIFIERS: &[Modifier] = &[Modifier::Control, Modifier::Shift, Modifier::Alt];

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Hotkey {
    pub key: Action,
    pub modifiers: Vec<Modifier>,
}

impl Hotkey {
    pub fn pressed(
        &self,
        keyboard_input: &Input<KeyCode>,
        mouse_input: &Input<MouseButton>,
    ) -> bool {
        self.modifiers_pressed(keyboard_input)
            && match self.key {
                Action::Button(button) => mouse_input.pressed(button),
                Action::Key(key) => keyboard_input.pressed(key),
                Action::Scroll(_) => false,
            }
    }

    pub fn just_pressed(
        &self,
        keyboard_input: &Input<KeyCode>,
        mouse_input: &Input<MouseButton>,
        mouse_wheel_events: &[&MouseWheel],
    ) -> bool {
        self.modifiers_pressed(keyboard_input)
            && match &self.key {
                Action::Button(button) => mouse_input.just_pressed(*button),
                Action::Key(key) => keyboard_input.just_pressed(*key),
                Action::Scroll(action) => mouse_wheel_events.iter().any(|event| match action {
                    MouseWheelAction::Up => event.y > 0.0,
                    MouseWheelAction::Down => event.y < 0.0,
                }),
            }
    }

    fn modifiers_pressed(&self, input: &Input<KeyCode>) -> bool {
        AVAILABLE_MODIFIERS.iter().all(|modifier| {
            let modifier_pressed = modifier
                .get_key_codes()
                .iter()
                .any(|key_code| input.pressed(*key_code));
            self.modifiers.contains(modifier) == modifier_pressed
        })
    }
}

impl Display for Hotkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key_name = match self.key {
            Action::Key(key) => format!("{:?}", key),
            _ => "".into(),
        };
        let content = AVAILABLE_MODIFIERS
            .iter()
            .filter(|modifier| self.modifiers.contains(modifier))
            .map(|modifier| match modifier {
                Modifier::Control => "ctrl",
                Modifier::Shift => "shift",
                Modifier::Alt => "alt",
            })
            .chain(iter::once(key_name.as_str()))
            .collect::<Vec<&str>>()
            .join("+");
        write!(f, "{}", content)
    }
}
