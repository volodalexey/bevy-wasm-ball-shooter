use bevy::{
    prelude::Resource,
    time::{Timer, TimerMode},
};

const POINTER_COOLDOWN_TIME: f32 = 1.0;

#[derive(Resource)]
pub struct PointerCooldown {
    pub timer: Timer,
    pub started: bool,
}

impl Default for PointerCooldown {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(POINTER_COOLDOWN_TIME, TimerMode::Repeating),
            started: false,
        }
    }
}

#[derive(Resource)]
pub struct LevelCounter(pub u32);
