use bevy::prelude::{Assets, Color, Commands, ResMut, StandardMaterial};

use crate::gameplay::ball::components::Species;

use super::resources::GameplayMaterials;

pub fn setup_resources(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(GameplayMaterials {
        red: materials.add(Species::Red.into()),
        blue: materials.add(Species::Blue.into()),
        green: materials.add(Species::Green.into()),
        yellow: materials.add(Species::Yellow.into()),
        white: materials.add(Species::White.into()),
        wall: materials.add(Color::AZURE.into()),
    })
}

pub fn cleanup_resources(mut commands: Commands) {
    commands.remove_resource::<GameplayMaterials>();
}
