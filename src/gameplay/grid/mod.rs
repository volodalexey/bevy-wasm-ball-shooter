use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnExit, OnUpdate, Plugin,
};

use self::{
    resources::Grid,
    systems::{cleanup_grid, display_grid_bounds, generate_grid, update_hex_coord_transforms},
};

use super::AppState;

mod constants;
pub mod resources;
pub mod systems;
pub mod utils;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid>()
            .add_system(generate_grid.in_schedule(OnEnter(AppState::Gameplay)))
            .add_systems(
                (update_hex_coord_transforms, display_grid_bounds)
                    .in_set(OnUpdate(AppState::Gameplay)),
            )
            .add_system(cleanup_grid.in_schedule(OnExit(AppState::Gameplay)));
    }
}
