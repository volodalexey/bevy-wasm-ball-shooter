use bevy::prelude::{AudioSource, Handle};

pub struct AudioEvent {
    pub clip: Handle<AudioSource>,
}

pub struct AudioLoopEvent {
    pub clip: Handle<AudioSource>,
}
