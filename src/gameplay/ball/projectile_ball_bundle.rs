use bevy::prelude::{PbrBundle, Res, Transform, Vec3};
use bevy_rapier3d::{
    prelude::{ActiveEvents, Ccd, Collider, LockedAxes, RigidBody, Sleeping, Velocity},
    render::ColliderDebugColor,
};

use crate::gameplay::{materials::resources::GameplayMaterials, meshes::resources::GameplayMeshes};

use super::{
    components::{ProjectileBall, Species},
    constants::INNER_RADIUS_COEFF,
};

pub struct ProjectileBallBundle;

impl ProjectileBallBundle {
    pub fn new(
        pos: Vec3,
        radius: f32,
        species: Species,
        gameplay_meshes: &Res<GameplayMeshes>,
        gameplay_materials: &Res<GameplayMaterials>,
    ) -> (
        PbrBundle,
        ProjectileBall,
        Species,
        Collider,
        RigidBody,
        LockedAxes,
        Velocity,
        ActiveEvents,
        Ccd,
        Sleeping,
        ColliderDebugColor,
    ) {
        (
            PbrBundle {
                mesh: gameplay_meshes.grid_ball.clone(),
                material: gameplay_materials.from_species(species),
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
            ProjectileBall {
                is_flying: false,
                is_ready_to_despawn: false,
            },
            species,
            Collider::ball(radius * INNER_RADIUS_COEFF),
            RigidBody::Dynamic,
            LockedAxes::TRANSLATION_LOCKED_Y,
            Velocity::default(),
            ActiveEvents::COLLISION_EVENTS,
            Ccd::enabled(),
            Sleeping::disabled(),
            ColliderDebugColor(species.into()),
        )
    }
}
