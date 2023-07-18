use bevy::prelude::{Handle, Mesh, Resource};

#[derive(Resource, Debug)]
pub struct GameplayMeshes {
    pub projectile_ball: Handle<Mesh>,
    pub grid_ball: Handle<Mesh>,
}
