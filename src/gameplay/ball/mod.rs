use bevy::prelude::{
    any_with_component, in_state, not, App, Condition, IntoSystemConfigs, OnExit, Plugin, Update,
};

use crate::components::AppState;

use self::{
    components::{ProjectileArrow, Species},
    events::{SnapProjectile, SpawnedBall},
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
            .add_event::<SpawnedBall>()
            .insert_resource(ProjectileBuffer(vec![Species::random_species()]))
            .add_systems(
                Update,
                (setup_projectile_arrow, setup_projectile_line).run_if(
                    in_state(AppState::Gameplay)
                        .and_then(not(any_with_component::<ProjectileArrow>())),
                ),
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
