use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::components::AppState;

use self::{
    events::SnapProjectile,
    resources::ProjectileBuffer,
    systems::{
        cleanup_projectile_arrow, cleanup_projectile_ball, cleanup_projectile_line,
        on_projectile_collisions_events, projectile_reload, setup_projectile_arrow,
        setup_projectile_line, shoot_projectile,
    },
};

pub mod components;
pub mod constants;
pub mod events;
pub mod grid_ball_bundle;
pub mod projectile_arrow_bundle;
pub mod projectile_ball_bundle;
pub mod projectile_line_bundle;
mod resources;
mod systems;
pub mod utils;
pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SnapProjectile>()
            .insert_resource(ProjectileBuffer(vec![]))
            .add_systems(
                OnEnter(AppState::Gameplay),
                (setup_projectile_arrow, setup_projectile_line),
            )
            .add_systems(
                Update,
                (
                    projectile_reload,
                    shoot_projectile,
                    on_projectile_collisions_events,
                )
                    .run_if(in_state(AppState::Gameplay)),
            )
            .add_systems(
                OnExit(AppState::Gameplay),
                (
                    cleanup_projectile_ball,
                    cleanup_projectile_arrow,
                    cleanup_projectile_line,
                ),
            );
    }
}
