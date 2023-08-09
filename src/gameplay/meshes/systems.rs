use bevy::prelude::{Assets, Commands, Mesh, ResMut};

use super::resources::GameplayMeshes;

pub fn setup_resources(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(GameplayMeshes::new(&mut meshes))
}

pub fn cleanup_resources(mut commands: Commands) {
    commands.remove_resource::<GameplayMeshes>();
}
