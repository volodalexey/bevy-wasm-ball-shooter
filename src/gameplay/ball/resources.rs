use bevy::prelude::Resource;

use super::components::Species;

#[derive(Resource)]
pub struct ProjectileBuffer(pub Vec<Species>);
