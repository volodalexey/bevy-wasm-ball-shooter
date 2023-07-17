use bevy::prelude::{
    in_state, run_once, App, Condition, IntoSystemConfigs, OnExit, Plugin, Update,
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

pub mod components;
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
            .add_systems(
                Update,
                generate_grid.run_if(in_state(AppState::Gameplay).and_then(run_once())),
            )
            .add_systems(
                Update,
                (
                    update_hex_coord_transforms,
                    display_grid_bounds,
                    move_down_and_spawn,
                )
                    .run_if(in_state(AppState::Gameplay)),
            )
            .add_systems(OnExit(AppState::Gameplay), cleanup_grid);
    }
}
