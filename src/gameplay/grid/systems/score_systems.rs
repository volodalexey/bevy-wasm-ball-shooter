use bevy::prelude::{Commands, EventReader, Res, ResMut};
use bevy_pkv::PkvStore;

use crate::{
    game_audio::utils::pkv_play_score_audio,
    gameplay::{events::UpdateScoreCounter, panels::resources::ScoreCounter},
    loading::audio_assets::AudioAssets,
};

pub fn update_score_counter(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    pkv: Res<PkvStore>,
    mut update_cooldown_counter_events: EventReader<UpdateScoreCounter>,
    mut score_counter: ResMut<ScoreCounter>,
) {
    if let Some(UpdateScoreCounter { score_add }) = update_cooldown_counter_events.iter().next() {
        if *score_add > 0 {
            pkv_play_score_audio(&mut commands, &audio_assets, &pkv);
            score_counter.0 += score_add;
        }
    }
    update_cooldown_counter_events.clear();
}
