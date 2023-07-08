pub mod events;
mod resources;
mod systems;

use bevy::prelude::{App, Plugin};

pub use self::resources::AudioAssets;
use self::{
    events::{AudioEvent, AudioLoopEvent},
    systems::{load_assets, on_audio_event, on_audio_loop_event},
};

pub struct AudioAssetsPlugin;

impl Plugin for AudioAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioAssets>()
            .add_event::<AudioEvent>()
            .add_event::<AudioLoopEvent>()
            .add_startup_system(load_assets)
            .add_system(on_audio_event)
            .add_system(on_audio_loop_event);
    }
}
