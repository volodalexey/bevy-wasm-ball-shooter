use bevy::{
    prelude::{shape, Assets, Bundle, Commands, Mesh, Res, ResMut, Transform, Vec2},
    sprite::MaterialMesh2dBundle,
};
use bevy_xpbd_2d::prelude::{
    CoefficientCombine, Collider, CollisionLayers, Friction, Position, Restitution, RigidBody,
};

use crate::gameplay::{
    constants::{WALL_SIDE_HEIGHT, WALL_SIDE_WIDTH, WALL_TOP_HEIGHT, WALL_TOP_WIDTH, WALL_Z_INDEX},
    materials::resources::GameplayMaterials,
    physics::layers::Layer,
};

use super::components::{TopWall, WallType};

pub struct WallBundle;

impl WallBundle {
    fn new(
        pos: Vec2,
        width: f32,
        height: f32,
        wall_type: WallType,
        meshes: &mut ResMut<Assets<Mesh>>,
        gameplay_materials: &Res<GameplayMaterials>,
    ) -> impl Bundle {
        (
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(width, height)).into())
                    .into(),
                material: gameplay_materials.side_wall.clone(),
                transform: Transform::from_translation(pos.extend(WALL_Z_INDEX)),
                ..Default::default()
            },
            wall_type,
            Collider::cuboid(width, height),
            Position(pos),
            CollisionLayers::new([Layer::Walls], [Layer::Grid]),
        )
    }

    pub fn spawn(
        commands: &mut Commands,
        pos: Vec2,
        wall_type: WallType,
        meshes: &mut ResMut<Assets<Mesh>>,
        gameplay_materials: &Res<GameplayMaterials>,
    ) {
        let is_side_wall = match wall_type {
            WallType::Left | WallType::Right => true,
            WallType::Top => false,
        };
        let mut entity_commands = commands.spawn(Self::new(
            pos,
            match is_side_wall {
                true => WALL_SIDE_WIDTH,
                false => WALL_TOP_WIDTH,
            },
            match is_side_wall {
                true => WALL_SIDE_HEIGHT,
                false => WALL_TOP_HEIGHT,
            },
            wall_type,
            meshes,
            gameplay_materials,
        ));
        if is_side_wall {
            entity_commands
                .insert(RigidBody::Static)
                .insert(Restitution {
                    coefficient: 1.0,
                    combine_rule: CoefficientCombine::Max,
                })
                .insert(Friction {
                    dynamic_coefficient: 0.0,
                    static_coefficient: 0.0,
                    combine_rule: CoefficientCombine::Min,
                });
        } else {
            entity_commands
                .insert(TopWall)
                .insert(RigidBody::Kinematic)
                .insert(Restitution {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombine::Multiply,
                });
        }
    }
}
