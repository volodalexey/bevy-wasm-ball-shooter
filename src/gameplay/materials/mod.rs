use bevy::prelude::{App, OnEnter, OnExit, Plugin};

use crate::components::AppState;

use self::systems::{cleanup_resources, setup_resources};

pub mod resources;
mod systems;

pub struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Gameplay), setup_resources)
            .add_systems(OnExit(AppState::Gameplay), cleanup_resources);
    }
}
