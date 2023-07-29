use bevy::{
    prelude::{shape, Assets, Mesh, Res, ResMut, Transform, Vec2, Vec3},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};
use bevy_rapier2d::prelude::{CoefficientCombineRule, Collider, Friction, Restitution, RigidBody};

use crate::gameplay::materials::resources::GameplayMaterials;

use super::components::WallType;

pub struct WallBundle;

pub const WALL_X_WIDTH: f32 = 0.4;

impl WallBundle {
    pub fn new(
        pos: Vec3,
        wall_type: WallType,
        meshes: &mut ResMut<Assets<Mesh>>,
        gameplay_materials: &Res<GameplayMaterials>,
        length: f32,
    ) -> (
        MaterialMesh2dBundle<ColorMaterial>,
        WallType,
        RigidBody,
        Collider,
        Restitution,
        Friction,
    ) {
        (
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(WALL_X_WIDTH, length)).into())
                    .into(),
                material: gameplay_materials.wall.clone(),
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
            wall_type,
            RigidBody::Fixed,
            Collider::cuboid(WALL_X_WIDTH / 2.0, length / 2.0),
            Restitution {
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Max,
            },
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
        )
    }
}
