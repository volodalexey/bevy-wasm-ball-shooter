use bevy::prelude::{
    apply_deferred, in_state, App, FixedTime, FixedUpdate, IntoSystemConfigs, OnEnter, OnExit,
    Plugin, Update,
};

use self::{
    resources::{ClusterCheckCooldown, CollisionSnapCooldown, CooldownMoveCounter, Grid},
    systems::{
        animation_systems::{animate_grid_ball_position, move_down_grid_balls},
        cluster_systems::find_and_remove_clusters,
        collision_systems::{check_collision_events, tick_collision_snap_cooldown_timer},
        confine_systems::confine_grid_balls,
        lifecycle_systems::{cleanup_grid, generate_grid, spawn_new_row},
        magnetic_systems::apply_magnetic_forces,
        projectile_systems::on_snap_projectile,
        resource_systems::update_grid_resources,
        score_systems::update_score_counter,
    },
};

use super::{constants::FIXED_TIMESTEP, AppState};

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
            .insert_resource(CooldownMoveCounter::default())
            .add_systems(OnEnter(AppState::GameplayInit), generate_grid)
            .add_systems(
                Update,
                (
                    move_down_grid_balls,
                    confine_grid_balls,
                    check_collision_events,
                    tick_collision_snap_cooldown_timer,
                    animate_grid_ball_position,
                    update_score_counter,
                    spawn_new_row,
                    apply_magnetic_forces,
                )
                    .run_if(in_state(AppState::Gameplay)),
            )
            .add_systems(
                Update,
                // snap projectile generate new grid ball, we need to use this ball in clusters, so wait after commands
                (on_snap_projectile, apply_deferred, find_and_remove_clusters).chain(),
            )
            .add_systems(FixedUpdate, update_grid_resources)
            .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
            .add_systems(OnExit(AppState::Gameplay), cleanup_grid);
    }
}
