use bevy::{
    audio::{Volume, VolumeLevel},
    prelude::{
        default, AudioBundle, AudioSink, AudioSinkPlayback, Commands, PlaybackSettings, Query, Res,
        With,
    },
};
use bevy_pkv::PkvStore;

use crate::loading::audio_assets::AudioAssets;

use super::{
    components::{MainSound, ShootSound},
    constants::SFX_SOUND_VOLUME_KEY,
};

pub fn setup_main_audio(
    commands: &mut Commands,
    audio_assets: &Res<AudioAssets>,
    volume: f32,
    paused: bool,
) {
    let playback_settings =
        PlaybackSettings::LOOP.with_volume(Volume::Relative(VolumeLevel::new(volume)));
    if paused {
        playback_settings.paused();
    }
    commands.spawn((
        AudioBundle {
            source: audio_assets.soundtrack.clone_weak(),
            settings: playback_settings,
            ..default()
        },
        MainSound {},
    ));
}

pub fn toggle_main_audio(query: &Query<&AudioSink, With<MainSound>>, volume: f32) {
    for sink in query.iter() {
        sink.set_volume(volume);
        if volume == 0.0 {
            sink.pause();
        } else {
            sink.play();
        }
    }
}

pub fn play_shoot_audio(commands: &mut Commands, audio_assets: &Res<AudioAssets>, volume: f32) {
    commands.spawn((
        AudioBundle {
            source: audio_assets.flying.clone_weak(),
            settings: PlaybackSettings::DESPAWN
                .with_volume(Volume::Relative(VolumeLevel::new(volume))),
            ..default()
        },
        ShootSound {},
    ));
}

pub fn pkv_play_shoot_audio(
    commands: &mut Commands,
    audio_assets: &Res<AudioAssets>,
    pkv: &Res<PkvStore>,
) {
    if let Ok(shoot_sound_volume) = pkv.get::<String>(SFX_SOUND_VOLUME_KEY) {
        if let Ok(shoot_sound_volume) = shoot_sound_volume.parse::<f32>() {
            if shoot_sound_volume > 0.0 {
                play_shoot_audio(commands, audio_assets, shoot_sound_volume);
            }
        }
    }
}

pub fn play_score_audio(commands: &mut Commands, audio_assets: &Res<AudioAssets>, volume: f32) {
    commands.spawn((
        AudioBundle {
            source: audio_assets.score.clone_weak(),
            settings: PlaybackSettings::DESPAWN
                .with_volume(Volume::Relative(VolumeLevel::new(volume))),
            ..default()
        },
        ShootSound {},
    ));
}

pub fn pkv_play_score_audio(
    commands: &mut Commands,
    audio_assets: &Res<AudioAssets>,
    pkv: &Res<PkvStore>,
) {
    if let Ok(sfx_sound_volume) = pkv.get::<String>(SFX_SOUND_VOLUME_KEY) {
        if let Ok(sfx_sound_volume) = sfx_sound_volume.parse::<f32>() {
            if sfx_sound_volume > 0.0 {
                play_score_audio(commands, audio_assets, sfx_sound_volume);
            }
        }
    }
}
