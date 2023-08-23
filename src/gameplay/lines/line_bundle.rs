use bevy::{
    prelude::{shape, Assets, Bundle, Mesh, Res, ResMut, Transform, Vec2},
    sprite::MaterialMesh2dBundle,
};
use bevy_xpbd_2d::prelude::{Collider, CollisionLayers, Position, RigidBody};

use crate::gameplay::{
    constants::{LINE_WIDTH, LINE_Z_INDEX},
    materials::resources::GameplayMaterials,
    physics::layers::Layer,
};

use super::components::LineType;

pub struct LineBundle;

impl LineBundle {
    pub fn new(
        width: f32,
        line_type: LineType,
        meshes: &mut ResMut<Assets<Mesh>>,
        gameplay_materials: &Res<GameplayMaterials>,
    ) -> impl Bundle {
        (
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(width, LINE_WIDTH)).into())
                    .into(),
                material: match line_type {
                    LineType::GridTop => gameplay_materials.grid_top_line.clone(),
                    LineType::GameOver => gameplay_materials.game_over_line.clone(),
                },
                transform: Transform::from_translation(Vec2::ZERO.extend(LINE_Z_INDEX)),
                ..Default::default()
            },
            Position::default(),
            RigidBody::Kinematic,
            Collider::cuboid(width / 2.0, LINE_WIDTH / 2.0),
            CollisionLayers::new([Layer::Lines], []),
            line_type,
        )
    }
}
