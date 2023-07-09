use bevy::prelude::{AssetServer, Audio, Commands, EventReader, PlaybackSettings, Res, ResMut};

use crate::loading::resources::AssetsLoading;

use super::{
    events::{AudioEvent, AudioLoopEvent},
    AudioAssets,
};

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

pub fn on_audio_event(audio: Res<Audio>, mut audio_events: EventReader<AudioEvent>) {
    if audio_events.is_empty() {
        return;
    }
    for event in audio_events.iter() {
        audio.play(event.clip.clone_weak());
    }
}

pub fn on_audio_loop_event(audio: Res<Audio>, mut audio_events: EventReader<AudioLoopEvent>) {
    if audio_events.is_empty() {
        return;
    }
    for event in audio_events.iter() {
        audio.play_with_settings(
            event.clip.clone_weak(),
            PlaybackSettings::LOOP.with_volume(0.5),
        );
    }
}
