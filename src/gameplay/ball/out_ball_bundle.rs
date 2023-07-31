use bevy::{
    prelude::{default, Res, Transform, Vec2, Vec3},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};
use bevy_rapier2d::prelude::{
    Collider, CollisionGroups, ExternalForce, Group, RigidBody, Velocity,
};

use crate::gameplay::{materials::resources::GameplayMaterials, meshes::resources::GameplayMeshes};

use super::{
    components::{OutBall, Species},
    constants::INNER_RADIUS_COEFF,
};

pub struct OutBallBundle;

impl OutBallBundle {
    pub fn new(
        pos: Vec2,
        radius: f32,
        species: Species,
        gameplay_meshes: &Res<GameplayMeshes>,
        gameplay_materials: &Res<GameplayMaterials>,
    ) -> (
        MaterialMesh2dBundle<ColorMaterial>,
        OutBall,
        Species,
        RigidBody,
        Collider,
        ExternalForce,
        Velocity,
        CollisionGroups,
    ) {
        (
            MaterialMesh2dBundle {
                mesh: gameplay_meshes.grid_ball.clone().into(),
                material: gameplay_materials.from_species(species),
                transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
                ..default()
            },
            OutBall::default(),
            species,
            RigidBody::Dynamic,
            Collider::ball(radius * INNER_RADIUS_COEFF),
            ExternalForce::default(),
            Velocity::default(),
            CollisionGroups::new(Group::GROUP_4, Group::NONE),
        )
    }
}
