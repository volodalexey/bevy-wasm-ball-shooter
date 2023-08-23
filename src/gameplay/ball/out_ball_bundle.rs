use bevy::{
    prelude::{default, Assets, Bundle, Res, ResMut, Vec2},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};
use bevy_xpbd_2d::prelude::{
    Collider, CollisionLayers, ExternalForce, LinearVelocity, Position, RigidBody,
};

use crate::gameplay::{
    constants::BALL_RADIUS, meshes::resources::GameplayMeshes, physics::layers::Layer,
};

use super::components::{OutBall, Species};

pub struct OutBallBundle;

impl OutBallBundle {
    pub fn new(
        pos: Vec2,
        species: Species,
        gameplay_meshes: &Res<GameplayMeshes>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        is_floating: bool,
    ) -> impl Bundle {
        (
            MaterialMesh2dBundle {
                mesh: gameplay_meshes.grid_ball.clone().into(),
                material: materials.add(species.into()),
                ..default()
            },
            match is_floating {
                true => OutBall::as_floating(),
                false => OutBall::as_fixed(),
            },
            species,
            RigidBody::Dynamic,
            Collider::ball(BALL_RADIUS),
            Position(pos),
            ExternalForce::default(),
            LinearVelocity::default(),
            CollisionLayers::new([Layer::Out], []),
        )
    }
}
