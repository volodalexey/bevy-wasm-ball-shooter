use bevy::prelude::{Assets, Color, ColorMaterial, Handle, ResMut, Resource};

use crate::gameplay::ball::components::Species;

#[derive(Resource, Debug)]
pub struct GameplayMaterials {
    pub red: Handle<ColorMaterial>,
    pub blue: Handle<ColorMaterial>,
    pub blau: Handle<ColorMaterial>,
    pub green: Handle<ColorMaterial>,
    pub yellow: Handle<ColorMaterial>,
    pub white: Handle<ColorMaterial>,
    pub purple: Handle<ColorMaterial>,
    pub side_wall: Handle<ColorMaterial>,
    pub game_over_line: Handle<ColorMaterial>,
    pub aim_target: Handle<ColorMaterial>,
    pub aim_line: Handle<ColorMaterial>,
}

impl Default for GameplayMaterials {
    fn default() -> Self {
        Self {
            red: Handle::default(),
            blue: Handle::default(),
            blau: Handle::default(),
            green: Handle::default(),
            yellow: Handle::default(),
            white: Handle::default(),
            purple: Handle::default(),
            side_wall: Handle::default(),
            game_over_line: Handle::default(),
            aim_target: Handle::default(),
            aim_line: Handle::default(),
        }
    }
}

impl GameplayMaterials {
    pub fn new(materials: &mut ResMut<Assets<ColorMaterial>>) -> Self {
        Self {
            red: materials.add(Species::Red.into()),
            blue: materials.add(Species::Blue.into()),
            blau: materials.add(Species::Blau.into()),
            green: materials.add(Species::Green.into()),
            yellow: materials.add(Species::Yellow.into()),
            white: materials.add(Species::White.into()),
            purple: materials.add(Species::Purple.into()),
            side_wall: materials.add(Color::AZURE.with_a(0.2).into()),
            game_over_line: materials.add(Color::RED.with_a(0.1).into()),
            aim_target: materials.add(Color::INDIGO.with_a(0.5).into()),
            aim_line: materials.add(Color::INDIGO.with_a(0.5).into()),
        }
    }

    pub fn from_species(&self, species: Species) -> Handle<ColorMaterial> {
        match species {
            Species::Red => self.red.clone(),
            Species::Blue => self.blue.clone(),
            Species::Blau => self.blau.clone(),
            Species::Green => self.green.clone(),
            Species::Yellow => self.yellow.clone(),
            Species::White => self.white.clone(),
            Species::Purple => self.purple.clone(),
        }
    }
}
