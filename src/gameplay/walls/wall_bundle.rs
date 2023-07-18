use bevy::prelude::{shape, Assets, Bundle, Mesh, PbrBundle, Res, ResMut, Transform, Vec3};
use bevy_rapier3d::prelude::{Collider, CollisionGroups, Group, RigidBody};

use crate::gameplay::materials::resources::GameplayMaterials;

use super::components::WallType;

#[derive(Bundle)]
pub struct WallBundle {
    pub pbr: PbrBundle,
    pub wall_type: WallType,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub collision_group: CollisionGroups,
}

pub const WALL_X_WIDTH: f32 = 5.0;
pub const WALL_Y: f32 = 1.0;
pub const WALL_Z: f32 = 100.0;

impl WallBundle {
    pub fn new(
        pos: Vec3,
        wall_type: WallType,
        meshes: &mut ResMut<Assets<Mesh>>,
        gameplay_materials: &Res<GameplayMaterials>,
    ) -> Self {
        Self {
            pbr: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(WALL_X_WIDTH, WALL_Y, WALL_Z))),
                material: gameplay_materials.wall.clone(),
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
            wall_type,
            rigid_body: RigidBody::Fixed,
            collider: Collider::cuboid(WALL_X_WIDTH / 2.0, WALL_Y / 2.0, WALL_Z / 2.0),
            collision_group: CollisionGroups::new(Group::GROUP_1, Group::ALL),
        }
    }
}
