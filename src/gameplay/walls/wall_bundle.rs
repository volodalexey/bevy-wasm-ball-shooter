use bevy::{
    prelude::{shape, Assets, Color, Mesh, Res, ResMut, Transform, Vec2, Vec3},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};
use bevy_rapier2d::{
    prelude::{
        CoefficientCombineRule, Collider, CollisionGroups, Friction, Group, Restitution, RigidBody,
    },
    render::ColliderDebugColor,
};

use crate::gameplay::materials::resources::GameplayMaterials;

use super::components::WallType;

pub struct WallBundle;

pub const WALL_X_WIDTH: f32 = 10.0;
pub const WALL_Y_HEIGHT: f32 = 2500.0;

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
        ColliderDebugColor,
        CollisionGroups,
    ) {
        (
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(WALL_X_WIDTH, length)).into())
                    .into(),
                material: gameplay_materials.side_wall.clone(),
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
            ColliderDebugColor(Color::AZURE.with_a(0.2)),
            CollisionGroups::new(Group::GROUP_1, Group::GROUP_2 | Group::GROUP_3),
        )
    }
}
