use bevy::prelude::{shape, Assets, Handle, Mesh, ResMut, Resource};

use crate::gameplay::constants::{BALL_RADIUS, NEXT_PROJECTILE_RADIUS};

#[derive(Resource, Debug)]
pub struct GameplayMeshes {
    pub projectile_ball: Handle<Mesh>,
    pub grid_ball: Handle<Mesh>,
    pub next_projectile_ball: Handle<Mesh>,
}

impl Default for GameplayMeshes {
    fn default() -> Self {
        Self {
            projectile_ball: Handle::default(),
            grid_ball: Handle::default(),
            next_projectile_ball: Handle::default(),
        }
    }
}

impl GameplayMeshes {
    pub fn new(meshes: &mut ResMut<Assets<Mesh>>) -> Self {
        Self {
            projectile_ball: meshes.add(shape::Circle::new(BALL_RADIUS).into()),
            grid_ball: meshes.add(shape::Circle::new(BALL_RADIUS).into()),
            next_projectile_ball: meshes.add(shape::Circle::new(NEXT_PROJECTILE_RADIUS).into()),
        }
    }
}
