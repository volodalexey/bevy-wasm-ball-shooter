use bevy::prelude::{
    apply_deferred, in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update,
};

use self::{
    resources::{CollisionSnapCooldown, Grid},
    systems::{
        animate_grid_ball_position, check_joints, check_projectile_out_of_grid, cleanup_grid,
        control_projectile_position, find_and_remove_clusters, generate_grid,
        on_projectile_collisions_events, on_snap_projectile, on_spawn_row,
        tick_collision_snap_cooldown_timer, update_hex_coord_transforms, update_score_counter,
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
                    tick_collision_snap_cooldown_timer,
                    animate_grid_ball_position,
                    control_projectile_position,
                    update_score_counter,
                    on_spawn_row,
                    check_joints,
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
