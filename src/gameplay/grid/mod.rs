use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnExit, OnUpdate, Plugin,
};

use self::{
    events::{MoveDownAndSpawn, UpdatePositions},
    resources::Grid,
    systems::{
        cleanup_grid, display_grid_bounds, generate_grid, move_down_and_spawn,
        update_hex_coord_transforms,
    },
};

use super::AppState;

mod constants;
pub mod events;
pub mod resources;
pub mod systems;
pub mod utils;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid>()
            .add_event::<UpdatePositions>()
            .add_event::<MoveDownAndSpawn>()
            .add_system(generate_grid.in_schedule(OnEnter(AppState::Gameplay)))
            .add_systems(
                (
                    update_hex_coord_transforms,
                    display_grid_bounds,
                    move_down_and_spawn,
                )
                    .in_set(OnUpdate(AppState::Gameplay)),
            )
            .add_system(cleanup_grid.in_schedule(OnExit(AppState::Gameplay)));
    }
}
