use bevy::prelude::{in_state, App, IntoSystemConfigs, OnExit, Plugin, Update};

use crate::components::AppState;

use self::{
    components::Species,
    events::{SnapProjectile, SpawnedBall},
    resources::ProjectileBuffer,
    systems::{
        aim_projectile, cleanup_projectile, on_projectile_collisions_events, projectile_reload,
        rotate_projectile,
    },
};

pub mod components;
pub mod constants;
pub mod events;
pub mod grid_ball_bundle;
pub mod projectile_ball_bundle;
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
                (rotate_projectile, projectile_reload, aim_projectile)
                    .run_if(in_state(AppState::Gameplay)),
            )
            .add_systems(
                Update,
                (on_projectile_collisions_events)
                    .chain()
                    .run_if(in_state(AppState::Gameplay)),
            )
            .add_systems(OnExit(AppState::Gameplay), cleanup_projectile);
    }
}
