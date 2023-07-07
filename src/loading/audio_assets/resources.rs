use bevy::prelude::{AudioSource, Handle, Resource};

#[derive(Resource, Debug)]
pub struct AudioAssets {
    pub flying: Handle<AudioSource>,
    pub soundtrack: Handle<AudioSource>,
    pub score: Handle<AudioSource>,
}

impl Default for AudioAssets {
    fn default() -> Self {
        Self {
            flying: Handle::default(),
            soundtrack: Handle::default(),
            score: Handle::default(),
        }
    }
}
