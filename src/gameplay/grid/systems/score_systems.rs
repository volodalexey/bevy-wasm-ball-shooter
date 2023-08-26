use bevy::prelude::{Commands, EventReader, EventWriter, Res, ResMut};
use bevy_pkv::PkvStore;

use crate::{
    game_audio::utils::pkv_play_score_audio,
    gameplay::{
        events::{MoveDownLastActive, UpdateScoreCounter},
        grid::resources::CooldownMoveCounter,
        panels::resources::{MoveCounter, ScoreCounter},
    },
    loading::audio_assets::AudioAssets,
};

pub fn update_score_counter(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    pkv: Res<PkvStore>,
    mut update_cooldown_counter_events: EventReader<UpdateScoreCounter>,
    mut cooldown_move_counter: ResMut<CooldownMoveCounter>,
    mut move_counter: ResMut<MoveCounter>,
    mut score_counter: ResMut<ScoreCounter>,
    mut writer_move_down_last_active: EventWriter<MoveDownLastActive>,
) {
    if let Some(UpdateScoreCounter {
        score_add,
        move_down_after,
    }) = update_cooldown_counter_events.iter().next()
    {
        if *score_add > 0 {
            pkv_play_score_audio(&mut commands, &audio_assets, &pkv);
            score_counter.0 += score_add;
        } else if cooldown_move_counter.init_value != 0 && *move_down_after {
            cooldown_move_counter.value -= 1;
            if cooldown_move_counter.value == 0 {
                move_counter.0 += 1;
                cooldown_move_counter.value = cooldown_move_counter.init_value;
                writer_move_down_last_active.send(MoveDownLastActive {});
            }
        }
    }
    update_cooldown_counter_events.clear();
}
