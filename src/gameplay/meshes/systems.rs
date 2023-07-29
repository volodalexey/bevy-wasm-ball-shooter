use bevy::prelude::{shape, Assets, Commands, Mesh, Res, ResMut};

use crate::gameplay::{ball::constants::INNER_RADIUS_COEFF, grid::resources::Grid};

use super::resources::GameplayMeshes;

pub fn setup_resources(mut commands: Commands, grid: Res<Grid>, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(GameplayMeshes {
        projectile_ball: meshes
            .add(shape::Circle::new(grid.layout.hex_size.y * INNER_RADIUS_COEFF).into()),
        grid_ball: meshes
            .add(shape::Circle::new(grid.layout.hex_size.y * INNER_RADIUS_COEFF).into()),
    })
}

pub fn cleanup_resources(mut commands: Commands) {
    commands.remove_resource::<GameplayMeshes>();
}
