use bevy::{
    prelude::{Res, Transform, Vec2, Vec3},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};
use bevy_rapier2d::{
    prelude::{ActiveEvents, Ccd, Collider, CollisionGroups, Group, RigidBody, Sleeping, Velocity},
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
        pos: Vec2,
        radius: f32,
        species: Species,
        gameplay_meshes: &Res<GameplayMeshes>,
        gameplay_materials: &Res<GameplayMaterials>,
    ) -> (
        MaterialMesh2dBundle<ColorMaterial>,
        ProjectileBall,
        Species,
        Collider,
        RigidBody,
        Velocity,
        ActiveEvents,
        Ccd,
        Sleeping,
        ColliderDebugColor,
        CollisionGroups,
    ) {
        (
            MaterialMesh2dBundle {
                mesh: gameplay_meshes.grid_ball.clone().into(),
                material: gameplay_materials.from_species(species),
                transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
                ..Default::default()
            },
            ProjectileBall {
                is_flying: false,
                is_ready_to_despawn: false,
            },
            species,
            Collider::ball(radius * INNER_RADIUS_COEFF),
            RigidBody::Dynamic,
            Velocity::default(),
            ActiveEvents::COLLISION_EVENTS,
            Ccd::enabled(),
            Sleeping::disabled(),
            ColliderDebugColor(species.into()),
            CollisionGroups::new(Group::GROUP_3, Group::GROUP_1 | Group::GROUP_2),
        )
    }
}
