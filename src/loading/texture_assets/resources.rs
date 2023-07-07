use bevy::prelude::{Handle, Image, Resource};

#[derive(Resource, Debug)]
pub struct TextureAssets {
    pub bevy: Handle<Image>,
}

impl Default for TextureAssets {
    fn default() -> Self {
        Self {
            bevy: Handle::default(),
        }
    }
}
