use bevy::prelude::{App, OnEnter, OnExit, Plugin};

use crate::AppState;

use self::systems::{cleanup_level_lines, setup_level_lines};

pub mod components;
mod line_bundle;
mod systems;

pub struct LinesPlugin;

impl Plugin for LinesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Gameplay), setup_level_lines)
            .add_systems(OnExit(AppState::Gameplay), cleanup_level_lines);
    }
}
