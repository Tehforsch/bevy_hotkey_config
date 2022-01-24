use bevy::prelude::KeyCode;
use bevy::prelude::MouseButton;
use serde::Deserialize;
use serde::Serialize;

use super::mouse_wheel_action::MouseWheelAction;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Action {
    Button(MouseButton),
    Scroll(MouseWheelAction),
    Key(KeyCode),
}
