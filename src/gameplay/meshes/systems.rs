use bevy::prelude::{shape, Assets, Commands, Mesh, ResMut};

use crate::gameplay::constants::BALL_RADIUS;

use super::resources::GameplayMeshes;

pub fn setup_resources(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(GameplayMeshes {
        projectile_ball: meshes.add(shape::Circle::new(BALL_RADIUS).into()),
        grid_ball: meshes.add(shape::Circle::new(BALL_RADIUS).into()),
    })
}

pub fn cleanup_resources(mut commands: Commands) {
    commands.remove_resource::<GameplayMeshes>();
}
