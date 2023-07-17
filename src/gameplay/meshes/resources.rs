use bevy::prelude::{Handle, Mesh, Resource};

#[derive(Resource, Debug)]
pub struct GameplayMeshes {
    pub projectile: Handle<Mesh>,
    pub grid_ball: Handle<Mesh>,
    pub wall: Handle<Mesh>,
}
