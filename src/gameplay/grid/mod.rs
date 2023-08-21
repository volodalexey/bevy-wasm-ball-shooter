use bevy::prelude::{
    apply_deferred, in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update,
};

use self::{
    resources::{ClusterCheckCooldown, CollisionSnapCooldown, Grid},
    systems::{
        animate_grid_ball_position, apply_magnetic_forces, check_collision_events,
        check_projectile_out_of_grid, cleanup_grid, find_and_remove_clusters, generate_grid,
        move_down_last_active, on_snap_projectile, on_spawn_row,
        tick_collision_snap_cooldown_timer, update_score_counter,
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
            .insert_resource(ClusterCheckCooldown::default())
            .add_systems(OnEnter(AppState::GameplayInit), generate_grid)
            .add_systems(
                Update,
                (
                    move_down_last_active,
                    check_projectile_out_of_grid,
                    check_collision_events,
                    tick_collision_snap_cooldown_timer,
                    animate_grid_ball_position,
                    update_score_counter,
                    on_spawn_row,
                    apply_magnetic_forces,
                )
                    .run_if(in_state(AppState::Gameplay)),
            )
            .add_systems(
                Update,
                // snap projectile generate new grid ball, we need to use this ball in clusters, so wait after commands
                (on_snap_projectile, apply_deferred, find_and_remove_clusters).chain(),
            )
            .add_systems(OnExit(AppState::Gameplay), cleanup_grid);
    }
}
