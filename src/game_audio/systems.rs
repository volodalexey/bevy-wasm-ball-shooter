use bevy::prelude::{Commands, Entity, Query, Res, With};
use bevy_pkv::PkvStore;

use crate::loading::audio_assets::AudioAssets;

use super::{
    components::MainSound,
    constants::MAIN_SOUND_VOLUME_KEY,
    utils::{cleanup_main_audio, setup_main_audio},
};

pub fn check_start_main_audio(
    mut commands: Commands,
    pkv: Res<PkvStore>,
    audio_assets: Res<AudioAssets>,
    query: Query<Entity, With<MainSound>>,
) {
    if let Ok(main_sound_volume) = pkv.get::<f32>(MAIN_SOUND_VOLUME_KEY) {
        if main_sound_volume == 0.0 {
            cleanup_main_audio(&mut commands, &query);
        } else if main_sound_volume > 0.0 {
            setup_main_audio(&mut commands, &audio_assets);
        }
    }
}
