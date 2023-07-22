use bevy::prelude::{App, OnEnter, OnExit, Plugin};

use crate::components::AppState;

use self::systems::{cleanup_resources, setup_resources};

pub mod resources;
mod systems;

pub struct MeshesPlugin;

impl Plugin for MeshesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameplayInit), setup_resources)
            .add_systems(OnExit(AppState::Gameplay), cleanup_resources);
    }
}
