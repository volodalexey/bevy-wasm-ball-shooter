use bevy::prelude::{Commands, Res};
use bevy_pkv::PkvStore;

use crate::{constants::LEVEL_KEY, gameplay::constants::START_LEVEL, resources::LevelCounter};

pub fn load_saved_level(mut commands: Commands, pkv: Res<PkvStore>) {
    if let Ok(level) = pkv.get::<String>(LEVEL_KEY) {
        if let Ok(level) = level.parse::<u32>() {
            commands.insert_resource(LevelCounter(level));
            return;
        }
    }
    commands.insert_resource(LevelCounter(START_LEVEL));
}
