use bevy::prelude::{shape, Assets, Mesh, PbrBundle, Res, ResMut, Transform, Vec3};
use bevy_rapier3d::prelude::{Collider, RigidBody};

use crate::gameplay::materials::resources::GameplayMaterials;

use super::components::WallType;

pub struct WallBundle;

pub const WALL_X_WIDTH: f32 = 0.4;
pub const WALL_Y: f32 = 2.0;

impl WallBundle {
    pub fn new(
        pos: Vec3,
        wall_type: WallType,
        meshes: &mut ResMut<Assets<Mesh>>,
        gameplay_materials: &Res<GameplayMaterials>,
        length: f32,
    ) -> (PbrBundle, WallType, RigidBody, Collider) {
        (
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(WALL_X_WIDTH, WALL_Y, length))),
                material: gameplay_materials.wall.clone(),
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
            wall_type,
            RigidBody::Fixed,
            Collider::cuboid(WALL_X_WIDTH / 2.0, WALL_Y / 2.0, length / 2.0),
        )
    }
}
