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
use crate::config;
use crate::modifier::Modifier;
use crate::modifier::AVAILABLE_MODIFIERS;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Hotkey {
    pub(crate) key: Action,
    pub(crate) modifiers: Vec<Modifier>,
}

impl Hotkey {
    pub(crate) fn pressed(
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

    pub(crate) fn just_pressed(
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
            .map(|modifier| modifier.to_str())
            .chain(iter::once(key_name.as_str()))
            .collect::<Vec<&str>>()
            .join(config::HOTKEY_SEPARATOR);
        write!(f, "{}", content)
    }
}
