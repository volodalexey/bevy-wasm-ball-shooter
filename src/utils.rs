use bevy::prelude::{ResMut, Vec2};
use bevy_pkv::PkvStore;

use crate::{
    constants::LEVEL_KEY,
    gameplay::constants::{MAX_LEVEL, START_LEVEL},
    resources::LevelCounter,
};

pub fn increment_level(level_counter: &mut LevelCounter, pkv: &mut ResMut<PkvStore>) {
    level_counter.0 += 1;
    if level_counter.0 > MAX_LEVEL {
        level_counter.0 = START_LEVEL
    }
    pkv.set_string(LEVEL_KEY, &level_counter.0.to_string())
        .expect("failed to save level");
}

pub fn from_grid_2d_to_2d(grid_2d: Vec2) -> Vec2 {
    return Vec2::new(grid_2d.x, -grid_2d.y);
}

pub fn from_2d_to_grid_2d(pos_2d: Vec2) -> Vec2 {
    return Vec2::new(pos_2d.x, -pos_2d.y);
}
