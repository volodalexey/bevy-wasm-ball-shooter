use bevy::prelude::{Commands, Res};
use bevy_pkv::PkvStore;

use crate::loading::audio_assets::AudioAssets;

use super::{constants::MAIN_SOUND_VOLUME_KEY, utils::setup_main_audio};

pub fn check_start_main_audio(
    mut commands: Commands,
    pkv: Res<PkvStore>,
    audio_assets: Res<AudioAssets>,
) {
    if let Ok(main_sound_volume) = pkv.get::<String>(MAIN_SOUND_VOLUME_KEY) {
        if let Ok(main_sound_volume) = main_sound_volume.parse::<f32>() {
            setup_main_audio(&mut commands, &audio_assets, main_sound_volume, false);
            return;
        }
    }
    setup_main_audio(&mut commands, &audio_assets, 0.0, true);
}
