use bevy::{
    audio::{Volume, VolumeLevel},
    prelude::{
        default, AudioBundle, Commands, DespawnRecursiveExt, Entity, PlaybackSettings, Query, Res,
        With,
    },
};

use crate::loading::audio_assets::AudioAssets;

use super::components::MainSound;

pub fn setup_main_audio(commands: &mut Commands, audio_assets: &Res<AudioAssets>) {
    commands.spawn((
        AudioBundle {
            source: audio_assets.soundtrack.clone_weak(),
            settings: PlaybackSettings::LOOP.with_volume(Volume::Relative(VolumeLevel::new(0.1))),
            ..default()
        },
        MainSound {},
    ));
}

pub fn cleanup_main_audio(commands: &mut Commands, query: &Query<Entity, With<MainSound>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
