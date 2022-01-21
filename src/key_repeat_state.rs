use bevy::core::Time;
use bevy::core::Timer;

use crate::hotkey_config::KeyRepeatSettings;

#[derive(Clone, Debug)]
pub struct KeyRepeatState {
    timer: Timer,
    in_fast_repeat: bool,

    initial_delay: f32,
    delay: f32,
}

impl KeyRepeatState {
    pub fn from_settings(key_repeat: &KeyRepeatSettings) -> Self {
        Self {
            timer: Timer::from_seconds(key_repeat.initial_delay, false),
            in_fast_repeat: false,
            initial_delay: key_repeat.initial_delay,
            delay: key_repeat.delay,
        }
    }

    pub fn key_repeated(&mut self) -> bool {
        let just_finished = self.timer.just_finished();
        if just_finished {
            if !self.in_fast_repeat {
                self.in_fast_repeat = true;
                self.timer = Timer::from_seconds(self.delay, true);
            }
            true
        } else {
            false
        }
    }

    pub fn tick(&mut self, time: &Time, is_pressed: bool) {
        self.timer.tick(time.delta());
        if !is_pressed {
            self.reset();
        }
    }

    pub fn reset(&mut self) {
        self.timer = Timer::from_seconds(self.initial_delay, false);
        self.in_fast_repeat = false;
    }
}
