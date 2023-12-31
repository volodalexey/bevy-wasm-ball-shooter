use bevy::{
    prelude::{default, Assets, Bundle, Res, ResMut, Transform, Vec2},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};
use bevy_xpbd_2d::prelude::{
    Collider, ColliderMassProperties, CollisionLayers, MassPropertiesBundle, Position, RigidBody,
};

use crate::gameplay::{
    constants::{BALL_RADIUS, OUT_BALL_Z_INDEX},
    meshes::resources::GameplayMeshes,
    physics::layers::Layer,
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
                transform: Transform::from_translation(pos.extend(OUT_BALL_Z_INDEX)),
                ..default()
            },
            match is_floating {
                true => OutBall::as_floating(),
                false => OutBall::as_fixed(),
            },
            species,
            RigidBody::Dynamic,
            Collider::ball(BALL_RADIUS),
            ColliderMassProperties::ZERO,
            MassPropertiesBundle::new_computed(&Collider::ball(1.0), 1.0),
            Position(pos),
            CollisionLayers::new([Layer::Out], []),
        )
    }
}
