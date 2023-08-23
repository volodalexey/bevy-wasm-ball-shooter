use bevy::{
    prelude::{shape, Assets, Bundle, Mesh, Res, ResMut, Vec2},
    sprite::MaterialMesh2dBundle,
};
use bevy_xpbd_2d::prelude::{
    CoefficientCombine, Collider, CollisionLayers, Friction, Position, Restitution, RigidBody,
};

use crate::gameplay::{
    constants::WALL_X_WIDTH, materials::resources::GameplayMaterials, physics::layers::Layer,
};

use super::components::WallType;

pub struct WallBundle;

impl WallBundle {
    pub fn new(
        pos: Vec2,
        wall_type: WallType,
        meshes: &mut ResMut<Assets<Mesh>>,
        gameplay_materials: &Res<GameplayMaterials>,
        length: f32,
    ) -> impl Bundle {
        (
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(WALL_X_WIDTH, length)).into())
                    .into(),
                material: gameplay_materials.side_wall.clone(),
                ..Default::default()
            },
            wall_type,
            RigidBody::Static,
            Collider::cuboid(WALL_X_WIDTH / 2.0, length / 2.0),
            Position(pos),
            Restitution {
                coefficient: 1.0,
                combine_rule: CoefficientCombine::Max,
            },
            Friction {
                dynamic_coefficient: 0.0,
                static_coefficient: 0.0,
                combine_rule: CoefficientCombine::Min,
            },
            CollisionLayers::new([Layer::Walls], [Layer::Projectile, Layer::Grid]),
        )
    }
}
