use bevy::{
    prelude::{Commands, Query, Res, ResMut, With, Without},
    text::Text,
    window::{PrimaryWindow, Window},
};

use crate::{
    loading::font_assets::FontAssets,
    resources::LevelCounter,
    ui::{
        components::ResponsiveText,
        resources::UIMenuTextColors,
        utils::{build_flex_full_row_evenly, build_responsive_text_component},
    },
};

use super::{
    components::{LevelText, ScoreText, TurnText},
    resources::{CooldownMoveCounter, MoveCounter, ScoreCounter, TurnCounter},
};

pub fn setup_resources(
    mut commands: Commands,
    mut turn_counter: ResMut<TurnCounter>,
    mut move_counter: ResMut<MoveCounter>,
    mut score_counter: ResMut<ScoreCounter>,
    level_counter: Res<LevelCounter>,
) {
    turn_counter.0 = 0;
    move_counter.0 = 0;
    score_counter.0 = 0;
    commands.insert_resource(CooldownMoveCounter::from_level(level_counter.0));
}

pub fn setup_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    text_colors: Res<UIMenuTextColors>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window_width = window_query.single().width();
    build_flex_full_row_evenly(&mut commands, |parent| {
        build_responsive_text_component(
            window_width,
            parent,
            "",
            &font_assets,
            &text_colors,
            (ScoreText {}, ResponsiveText {}),
        );
        build_responsive_text_component(
            window_width,
            parent,
            "",
            &font_assets,
            &text_colors,
            (TurnText {}, ResponsiveText {}),
        );
        build_responsive_text_component(
            window_width,
            parent,
            "",
            &font_assets,
            &text_colors,
            (LevelText {}, ResponsiveText {}),
        );
    });
}

pub fn update_ui(
    score_counter: Res<ScoreCounter>,
    mut score_text_query: Query<
        &mut Text,
        (With<ScoreText>, Without<TurnText>, Without<LevelText>),
    >,
    turn_counter: Res<TurnCounter>,
    cooldown_move_counter: Res<CooldownMoveCounter>,
    mut turn_text_query: Query<&mut Text, (With<TurnText>, Without<ScoreText>, Without<LevelText>)>,
    level_counter: Res<LevelCounter>,
    mut level_text_query: Query<
        &mut Text,
        (With<LevelText>, Without<ScoreText>, Without<TurnText>),
    >,
) {
    for mut score_text in &mut score_text_query {
        score_text.sections[0].value = format!("Очки: {:?} ", score_counter.0);
    }
    for mut turn_text in &mut turn_text_query {
        turn_text.sections[0].value = format!(
            "Ходов: {} ({})",
            turn_counter.0, cooldown_move_counter.value
        );
    }
    for mut level_text in &mut level_text_query {
        level_text.sections[0].value = format!("Уровень: {}", level_counter.0);
    }
}
