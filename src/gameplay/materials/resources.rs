use bevy::prelude::{ColorMaterial, Handle, Resource};

use crate::gameplay::ball::components::Species;

#[derive(Resource, Debug)]
pub struct GameplayMaterials {
    pub red: Handle<ColorMaterial>,
    pub blue: Handle<ColorMaterial>,
    pub green: Handle<ColorMaterial>,
    pub yellow: Handle<ColorMaterial>,
    pub white: Handle<ColorMaterial>,
    pub wall: Handle<ColorMaterial>,
    pub arrow_end: Handle<ColorMaterial>,
    pub arrow_line: Handle<ColorMaterial>,
}

impl GameplayMaterials {
    pub fn from_species(&self, species: Species) -> Handle<ColorMaterial> {
        match species {
            Species::Red => self.red.clone(),
            Species::Blue => self.blue.clone(),
            Species::Green => self.green.clone(),
            Species::Yellow => self.yellow.clone(),
            Species::White => self.white.clone(),
        }
    }
}
