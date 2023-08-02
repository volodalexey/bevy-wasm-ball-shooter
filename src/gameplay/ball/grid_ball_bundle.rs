use bevy::{
    prelude::{default, Res, Transform, Vec2, Vec3},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};
use bevy_rapier2d::{
    prelude::{Collider, CollisionGroups, Group, RigidBody, Velocity},
    render::ColliderDebugColor,
};
use hexx::Hex;

use crate::gameplay::{
    constants::BALL_RADIUS, materials::resources::GameplayMaterials,
    meshes::resources::GameplayMeshes,
};

use super::components::{GridBall, Species};

pub struct GridBallBundle;

impl GridBallBundle {
    pub fn new(
        pos: Vec2,
        species: Species,
        gameplay_meshes: &Res<GameplayMeshes>,
        gameplay_materials: &Res<GameplayMaterials>,
        hex: Hex,
        rigid_body: RigidBody,
    ) -> (
        MaterialMesh2dBundle<ColorMaterial>,
        GridBall,
        Species,
        RigidBody,
        Collider,
        ColliderDebugColor,
        CollisionGroups,
        Velocity,
    ) {
        (
            MaterialMesh2dBundle {
                mesh: gameplay_meshes.grid_ball.clone().into(),
                material: gameplay_materials.from_species(species),
                transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
                ..default()
            },
            GridBall { hex },
            species,
            rigid_body,
            Collider::ball(BALL_RADIUS),
            ColliderDebugColor(species.into()),
            CollisionGroups::new(Group::GROUP_2, Group::GROUP_1 | Group::GROUP_3),
            Velocity::default(),
        )
    }
}
