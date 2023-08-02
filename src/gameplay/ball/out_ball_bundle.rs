use bevy::{
    prelude::{default, Assets, Res, ResMut, Transform, Vec2, Vec3},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};
use bevy_rapier2d::prelude::{
    Collider, CollisionGroups, ExternalForce, Group, RigidBody, Velocity,
};

use crate::gameplay::{constants::BALL_RADIUS, meshes::resources::GameplayMeshes};

use super::components::{OutBall, Species};

pub struct OutBallBundle;

impl OutBallBundle {
    pub fn new(
        pos: Vec2,
        species: Species,
        gameplay_meshes: &Res<GameplayMeshes>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
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
                material: materials.add(species.into()),
                transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
                ..default()
            },
            OutBall::default(),
            species,
            RigidBody::Dynamic,
            Collider::ball(BALL_RADIUS),
            ExternalForce::default(),
            Velocity::default(),
            CollisionGroups::new(Group::GROUP_4, Group::NONE),
        )
    }
}
