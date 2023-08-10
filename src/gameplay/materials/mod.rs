use bevy::prelude::{App, OnEnter, Plugin};

use crate::components::AppState;

use self::systems::setup_resources;

pub mod resources;
mod systems;

pub struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), setup_resources);
    }
}
