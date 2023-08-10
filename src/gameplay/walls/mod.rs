use bevy::prelude::{App, OnEnter, OnExit, Plugin};

use crate::AppState;

use self::systems::{cleanup_level_walls, setup_level_walls};

pub mod components;
mod systems;
mod wall_bundle;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Gameplay), setup_level_walls)
            .add_systems(OnExit(AppState::Gameplay), cleanup_level_walls);
    }
}
