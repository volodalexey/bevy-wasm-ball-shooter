use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use self::{
    resources::{CollisionSnapCooldown, Grid},
    systems::{
        animate_grid_ball, check_projectile_out_of_grid, cleanup_grid, control_projectile_position,
        generate_grid, on_projectile_collisions_events, on_snap_projectile, on_spawn_row,
        tick_collision_snap_cooldown_timer, update_cooldown_move_counter,
        update_hex_coord_transforms,
    },
};

use super::AppState;

pub mod components;
pub mod resources;
pub mod systems;
pub mod utils;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid>()
            .insert_resource(CollisionSnapCooldown::default())
            .add_systems(OnEnter(AppState::GameplayInit), generate_grid)
            .add_systems(
                Update,
                (
                    update_hex_coord_transforms,
                    check_projectile_out_of_grid,
                    on_projectile_collisions_events,
                    on_snap_projectile,
                    tick_collision_snap_cooldown_timer,
                    animate_grid_ball,
                    control_projectile_position,
                    update_cooldown_move_counter,
                    on_spawn_row,
                )
                    .run_if(in_state(AppState::Gameplay)),
            )
            .add_systems(OnExit(AppState::Gameplay), cleanup_grid);
    }
}
