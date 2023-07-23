use bevy::prelude::{Event, Vec2};

use super::components::Species;

#[derive(Event)]
pub struct SnapProjectile {
    pub out_of_bounds: bool,
    pub pos: Vec2,
    pub species: Species,
}
