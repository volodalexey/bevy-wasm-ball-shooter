use bevy::{
    prelude::{shape, Assets, Mesh, Res, ResMut, Transform, Vec2},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};
use bevy_rapier2d::prelude::{Collider, CollisionGroups, Group, RigidBody};

use crate::gameplay::{
    constants::{LINE_WIDTH, LINE_Z_INDEX},
    materials::resources::GameplayMaterials,
};

use super::components::LineType;

pub struct LineBundle;

impl LineBundle {
    pub fn new(
        width: f32,
        line_type: LineType,
        meshes: &mut ResMut<Assets<Mesh>>,
        gameplay_materials: &Res<GameplayMaterials>,
    ) -> (
        MaterialMesh2dBundle<ColorMaterial>,
        RigidBody,
        Collider,
        CollisionGroups,
        LineType,
    ) {
        (
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(width, LINE_WIDTH)).into())
                    .into(),
                material: match line_type {
                    LineType::GridTop | LineType::GridBottom => {
                        gameplay_materials.grid_line.clone()
                    }
                    LineType::GameOver => gameplay_materials.game_over_line.clone(),
                },
                transform: Transform::from_translation(Vec2::ZERO.extend(LINE_Z_INDEX)),
                ..Default::default()
            },
            RigidBody::KinematicPositionBased,
            Collider::cuboid(width / 2.0, LINE_WIDTH / 2.0),
            CollisionGroups::new(
                match line_type {
                    LineType::GridTop => Group::GROUP_5,
                    LineType::GameOver | LineType::GridBottom => Group::GROUP_6,
                },
                Group::GROUP_5,
            ), // ray cast won't work with Group::NONE
            line_type,
        )
    }
}
