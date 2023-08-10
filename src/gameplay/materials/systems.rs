use bevy::{
    prelude::{Assets, Commands, ResMut},
    sprite::ColorMaterial,
};

use super::resources::GameplayMaterials;

pub fn setup_resources(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(GameplayMaterials::new(&mut materials));
}
