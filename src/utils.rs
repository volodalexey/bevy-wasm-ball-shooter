use bevy::prelude::ResMut;
use bevy_pkv::PkvStore;

use crate::{
    constants::TOTAL_COLORS_KEY,
    gameplay::constants::{MAX_LEVEL, START_LEVEL},
    resources::LevelCounter,
};

pub fn increment_level(level_counter: &mut LevelCounter, pkv: &mut ResMut<PkvStore>) {
    level_counter.0 += 1;
    if level_counter.0 > MAX_LEVEL {
        level_counter.0 = START_LEVEL
    }
    pkv.set_string(TOTAL_COLORS_KEY, &level_counter.0.to_string())
        .expect("failed to save level");
}
