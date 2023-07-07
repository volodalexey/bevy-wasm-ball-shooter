use bevy::prelude::Resource;

use crate::gameplay::ball::Species;

#[derive(Resource)]
pub struct ProjectileBuffer(pub Vec<Species>);
