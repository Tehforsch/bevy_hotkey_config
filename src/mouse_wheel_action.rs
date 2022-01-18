use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MouseWheelAction {
    Up,
    Down,
}
