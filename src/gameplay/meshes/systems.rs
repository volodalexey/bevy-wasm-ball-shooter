use bevy::prelude::{shape, Assets, Commands, Mesh, Res, ResMut};

use crate::gameplay::{ball::constants::INNER_RADIUS_COEFF, grid::resources::Grid};

use super::resources::GameplayMeshes;

pub fn setup_resources(mut commands: Commands, grid: Res<Grid>, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(GameplayMeshes {
        projectile_ball: meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: grid.layout.hex_size.y * INNER_RADIUS_COEFF,
                subdivisions: 1,
            })
            .expect("Unable to generate IcoSphere"),
        ),
        grid_ball: meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: grid.layout.hex_size.y * INNER_RADIUS_COEFF,
                subdivisions: 1,
            })
            .expect("Unable to generate IcoSphere"),
        ),
    })
}

pub fn cleanup_resources(mut commands: Commands) {
    commands.remove_resource::<GameplayMeshes>();
}
