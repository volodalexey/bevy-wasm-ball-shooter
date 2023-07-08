use bevy::prelude::{
    default, App, IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnExit, OnUpdate, Plugin, Vec2,
};

use self::{
    resources::Grid,
    systems::{cleanup_grid, display_grid_bounds, generate_grid, update_hex_coord_transforms},
};

use super::{
    hex::{Layout, Orientation},
    AppState,
};

pub mod resources;
pub mod systems;
pub mod utils;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid {
            layout: Layout {
                orientation: Orientation::pointy().clone(),
                origin: Vec2::new(0.0, 0.0),
                size: Vec2::new(1.0, 1.0),
            },
            ..default()
        })
        .add_system(generate_grid.in_schedule(OnEnter(AppState::Gameplay)))
        .add_systems(
            (update_hex_coord_transforms, display_grid_bounds).in_set(OnUpdate(AppState::Gameplay)),
        )
        .add_system(cleanup_grid.in_schedule(OnExit(AppState::Gameplay)));
    }
}
