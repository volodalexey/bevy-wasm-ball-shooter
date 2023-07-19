use bevy::prelude::{Handle, Resource, StandardMaterial};

use crate::gameplay::ball::components::Species;

#[derive(Resource, Debug)]
pub struct GameplayMaterials {
    pub red: Handle<StandardMaterial>,
    pub blue: Handle<StandardMaterial>,
    pub green: Handle<StandardMaterial>,
    pub yellow: Handle<StandardMaterial>,
    pub white: Handle<StandardMaterial>,
    pub wall: Handle<StandardMaterial>,
    pub arrow_end: Handle<StandardMaterial>,
    pub arrow_line: Handle<StandardMaterial>,
}

impl GameplayMaterials {
    pub fn from_species(&self, species: Species) -> Handle<StandardMaterial> {
        match species {
            Species::Red => self.red.clone(),
            Species::Blue => self.blue.clone(),
            Species::Green => self.green.clone(),
            Species::Yellow => self.yellow.clone(),
            Species::White => self.white.clone(),
        }
    }
}
