pub mod events;
mod resources;
mod systems;

use bevy::prelude::{App, AssetServer, Commands, Plugin, ResMut};

pub use self::resources::AudioAssets;
use self::{
    events::{AudioEvent, AudioLoopEvent},
    systems::{on_audio_event, on_audio_loop_event},
};

use super::resources::AssetsLoading;

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

pub fn load_assets(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    let assets = AudioAssets {
        flying: asset_server.load("audio/flying.ogg"),
        soundtrack: asset_server.load("audio/soundtrack.ogg"),
        score: asset_server.load("audio/score.ogg"),
    };

    loading.0.push(assets.flying.clone_weak_untyped());
    loading.0.push(assets.soundtrack.clone_weak_untyped());
    loading.0.push(assets.score.clone_weak_untyped());

    commands.insert_resource(assets);
}
