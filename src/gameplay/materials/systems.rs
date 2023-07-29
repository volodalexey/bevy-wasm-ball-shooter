use bevy::{
    prelude::{Assets, Color, Commands, NextState, ResMut},
    sprite::ColorMaterial,
};

use crate::{components::AppState, gameplay::ball::components::Species};

use super::resources::GameplayMaterials;

pub fn setup_resources(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    commands.insert_resource(GameplayMaterials {
        red: materials.add(Species::Red.into()),
        blue: materials.add(Species::Blue.into()),
        green: materials.add(Species::Green.into()),
        yellow: materials.add(Species::Yellow.into()),
        white: materials.add(Species::White.into()),
        wall: materials.add(Color::AZURE.with_a(0.2).into()),
        arrow_end: materials.add(Color::INDIGO.with_a(0.5).into()),
        arrow_line: materials.add(Color::INDIGO.with_a(0.5).into()),
    });

    app_state_next_state.set(AppState::Gameplay);
}

pub fn cleanup_resources(mut commands: Commands) {
    commands.remove_resource::<GameplayMaterials>();
}
