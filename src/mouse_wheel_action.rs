use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum MouseWheelAction {
    Up,
    Down,
}
