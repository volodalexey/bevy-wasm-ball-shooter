use bevy::{
    prelude::{Commands, Query, Res, ResMut, With, Without},
    text::Text,
    window::{PrimaryWindow, Window},
};

use crate::{
    components::AppState,
    gameplay::{
        constants::FILL_PLAYGROUND_ROWS,
        grid::resources::{CooldownMoveCounter, Grid},
    },
    loading::{font_assets::FontAssets, sprite_assets::SpriteAssets},
    resources::LevelCounter,
    ui::{
        components::NextStateButton,
        resources::{ColorType, UIMenuButtonColors, UIMenuTextColors},
        utils::{
            button_utils::append_middle_icon_button, flex_utils::build_flex_full_row_evenly,
            text_utils::append_responsive_text,
        },
    },
};

use super::{
    components::{LevelText, ScoreText, TurnText},
    resources::{MoveCounter, ScoreCounter, TurnCounter},
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
    sprite_assets: Res<SpriteAssets>,
    text_colors: Res<UIMenuTextColors>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    button_colors: Res<UIMenuButtonColors>,
) {
    let window_width = window_query.single().width();
    build_flex_full_row_evenly(&mut commands, |parent| {
        append_middle_icon_button(
            parent,
            NextStateButton {
                color_type: ColorType::Gray,
                next_state: AppState::GameOver,
            },
            &ColorType::Gray,
            &sprite_assets,
            &button_colors,
            false,
        );
        append_responsive_text(
            parent,
            window_width,
            "",
            &font_assets,
            &text_colors,
            Some(ScoreText {}),
        );
        append_responsive_text(
            parent,
            window_width,
            "",
            &font_assets,
            &text_colors,
            Some(TurnText {}),
        );
        append_responsive_text(
            parent,
            window_width,
            "",
            &font_assets,
            &text_colors,
            Some(LevelText {}),
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
    move_counter: Res<MoveCounter>,
    cooldown_move_counter: Res<CooldownMoveCounter>,
    mut turn_text_query: Query<&mut Text, (With<TurnText>, Without<ScoreText>, Without<LevelText>)>,
    level_counter: Res<LevelCounter>,
    mut level_text_query: Query<
        &mut Text,
        (With<LevelText>, Without<ScoreText>, Without<TurnText>),
    >,
    grid: Res<Grid>,
) {
    for mut score_text in &mut score_text_query {
        score_text.sections[0].value = format!("Очки: {:?} ", score_counter.0);
    }
    for mut turn_text in &mut turn_text_query {
        let left_spawn_count = grid.init_rows - FILL_PLAYGROUND_ROWS - move_counter.0 as i32 - 1;
        turn_text.sections[0].value = format!(
            "Ходов: {}/{} ({})",
            turn_counter.0,
            match left_spawn_count > 0 {
                true => left_spawn_count,
                false => 0,
            },
            cooldown_move_counter.value
        );
    }
    for mut level_text in &mut level_text_query {
        level_text.sections[0].value = format!("Уровень: {}", level_counter.0);
    }
}
