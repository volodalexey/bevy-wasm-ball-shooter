use bevy::prelude::{App, IntoSystemAppConfig, IntoSystemConfigs, OnExit, OnUpdate, Plugin};

use crate::components::AppState;

use self::{
    events::{SnapProjectile, SpawnedBall},
    resources::ProjectileBuffer,
    systems::{
        aim_projectile, bounce_on_world_bounds, cleanup_projectile,
        on_projectile_collisions_events, projectile_reload, rotate_projectile,
    },
};

use super::ball::random_species;

mod bundles;
pub mod components;
mod constants;
pub mod events;
mod resources;
mod systems;
pub mod utils;
pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SnapProjectile>()
            .add_event::<SpawnedBall>()
            .insert_resource(ProjectileBuffer(vec![random_species()]))
            .add_systems(
                (rotate_projectile, projectile_reload, aim_projectile)
                    .in_set(OnUpdate(AppState::Gameplay)),
            )
            .add_systems(
                (bounce_on_world_bounds, on_projectile_collisions_events)
                    .chain()
                    .in_set(OnUpdate(AppState::Gameplay)),
            )
            .add_system(cleanup_projectile.in_schedule(OnExit(AppState::Gameplay)));
    }
}
