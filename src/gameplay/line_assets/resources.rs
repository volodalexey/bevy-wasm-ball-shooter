use bevy::prelude::{Handle, Mesh, Resource, StandardMaterial};

#[derive(Resource, Debug)]
pub struct LineAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl Default for LineAssets {
    fn default() -> Self {
        Self {
            mesh: Handle::default(),
            material: Handle::default(),
        }
    }
}
