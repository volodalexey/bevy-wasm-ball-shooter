use bevy::prelude::{Handle, Image, Resource};

#[derive(Resource, Debug)]
pub struct SpriteAssets {
    pub arrow_up_right_from_square: Handle<Image>,
}

impl Default for SpriteAssets {
    fn default() -> Self {
        Self {
            arrow_up_right_from_square: Handle::default(),
        }
    }
}
