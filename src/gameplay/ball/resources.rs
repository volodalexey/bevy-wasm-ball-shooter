use bevy::{
    prelude::Resource,
    time::{Timer, TimerMode},
};

use crate::gameplay::constants::PROJECTILE_RELOAD_TIME;

use super::components::Species;

#[derive(Resource)]
pub struct ProjectileHelper {
    pub reload_timer: Timer,
    pub buffer: Vec<Species>,
}

impl Default for ProjectileHelper {
    fn default() -> Self {
        Self {
            reload_timer: Timer::from_seconds(PROJECTILE_RELOAD_TIME, TimerMode::Repeating),
            buffer: Vec::default(),
        }
    }
}
