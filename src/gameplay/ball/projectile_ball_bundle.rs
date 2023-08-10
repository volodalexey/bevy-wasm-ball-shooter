use bevy::{
    prelude::{default, Res, Transform, Vec2, Vec3},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};
use bevy_rapier2d::{
    prelude::{
        ActiveEvents, Ccd, Collider, CollisionGroups, ExternalImpulse, Group, RigidBody, Sleeping,
        Velocity,
    },
    render::ColliderDebugColor,
};

use crate::gameplay::{
    constants::BALL_RADIUS, materials::resources::GameplayMaterials,
    meshes::resources::GameplayMeshes,
};

use super::components::{Aim, NextProjectileBall, ProjectileBall, Species};

pub struct ProjectileBallBundle;

impl ProjectileBallBundle {
    pub fn new(
        pos: Vec2,
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
        ExternalImpulse,
        Aim,
    ) {
        (
            MaterialMesh2dBundle {
                mesh: gameplay_meshes.projectile_ball.clone().into(),
                material: gameplay_materials.from_species(species),
                transform: Transform::from_translation(pos.extend(0.0)),
                ..default()
            },
            ProjectileBall::default(),
            species,
            Collider::ball(BALL_RADIUS),
            RigidBody::Dynamic,
            Velocity::default(),
            ActiveEvents::COLLISION_EVENTS,
            Ccd::enabled(),
            Sleeping::disabled(),
            ColliderDebugColor(species.into()),
            CollisionGroups::new(Group::GROUP_3, Group::GROUP_1 | Group::GROUP_2),
            ExternalImpulse::default(),
            Aim::default(),
        )
    }
}

pub struct NextProjectileBallBundle;

impl NextProjectileBallBundle {
    pub fn new(
        pos: Vec2,
        species: Species,
        gameplay_meshes: &Res<GameplayMeshes>,
        gameplay_materials: &Res<GameplayMaterials>,
    ) -> (
        MaterialMesh2dBundle<ColorMaterial>,
        NextProjectileBall,
        Species,
    ) {
        (
            MaterialMesh2dBundle {
                mesh: gameplay_meshes.next_projectile_ball.clone().into(),
                material: gameplay_materials.from_species(species),
                transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
                ..default()
            },
            NextProjectileBall {},
            species,
        )
    }
}
