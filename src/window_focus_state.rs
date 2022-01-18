use bevy::prelude::*;

#[derive(Clone, Copy)]
pub(super) enum WindowFocusState {
    InFocus,
    OutOfFocusOrWaitingForInputRelease,
}

impl Default for WindowFocusState {
    fn default() -> Self {
        WindowFocusState::InFocus
    }
}

impl WindowFocusState {
    pub fn interaction_allowed(
        &mut self,
        window: &Window,
        keyboard_input: &Input<KeyCode>,
        mouse_input: &Input<MouseButton>,
    ) -> bool {
        use WindowFocusState::*;

        if window.is_focused() {
            match self {
                InFocus => true,
                OutOfFocusOrWaitingForInputRelease
                    if keyboard_input.get_pressed().len() > 0
                        || mouse_input.get_pressed().len() > 0 =>
                {
                    // The window is now focused, but a key or mouse button is still pressed.
                    // This might have been the key/button that was used to refocus the window.
                    // Wait for it to be released.
                    false
                }
                OutOfFocusOrWaitingForInputRelease => {
                    // All buttons have been released, give back control.
                    *self = InFocus;
                    true
                }
            }
        } else {
            *self = OutOfFocusOrWaitingForInputRelease;
            false
        }
    }
}
