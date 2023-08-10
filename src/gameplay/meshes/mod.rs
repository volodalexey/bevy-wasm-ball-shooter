use bevy::prelude::{App, OnEnter, Plugin};

use crate::components::AppState;

use self::systems::setup_resources;

pub mod resources;
mod systems;

pub struct MeshesPlugin;

impl Plugin for MeshesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), setup_resources);
    }
}
