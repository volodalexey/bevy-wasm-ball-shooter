use bevy::{
    prelude::{Commands, Query, Res, ResMut, With, Without},
    text::Text,
    window::{PrimaryWindow, Window},
};
use bevy_pkv::PkvStore;

use crate::{
    components::AppState,
    constants::{
        INIT_ROWS_KEY, MOVE_DOWN_AFTER_KEY, TOTAL_COLORS_KEY, TOTAL_COLUMNS_KEY, TOTAL_ROWS_KEY,
    },
    gameplay::grid::resources::{CooldownMoveCounter, Grid},
    loading::{font_assets::FontAssets, sprite_assets::SpriteAssets},
    settings_menu::utils::{
        colors_utils::read_total_colors,
        columns_utils::read_init_cols,
        move_down_utils::read_move_down,
        rows_utils::{read_init_rows, read_total_rows},
    },
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
    resources::{MoveDownCounter, ScoreCounter, SpawnRowsLeft, TurnCounter},
};

pub fn setup_resources(
    mut commands: Commands,
    mut turn_counter: ResMut<TurnCounter>,
    mut move_counter: ResMut<MoveDownCounter>,
    mut score_counter: ResMut<ScoreCounter>,
    mut spawn_rows_left: ResMut<SpawnRowsLeft>,
    pkv: Res<PkvStore>,
    mut grid: ResMut<Grid>,
) {
    turn_counter.0 = 0;
    move_counter.0 = 0;
    score_counter.0 = 0;
    let move_down_after = read_move_down(MOVE_DOWN_AFTER_KEY, &pkv);
    commands.insert_resource(CooldownMoveCounter::init(move_down_after));

    grid.total_colors = read_total_colors(TOTAL_COLORS_KEY, &pkv);
    grid.init_cols = read_init_cols(TOTAL_COLUMNS_KEY, &pkv);
    grid.init_rows = read_init_rows(INIT_ROWS_KEY, &pkv);
    grid.total_rows = read_total_rows(TOTAL_ROWS_KEY, &pkv);

    grid.calc_last_active_row();

    let left_rows = grid.total_rows as i32 - grid.init_rows as i32;
    spawn_rows_left.0 = match left_rows > 0 {
        true => left_rows as u32,
        false => 0,
    };
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
    spawn_rows_left: Res<SpawnRowsLeft>,
    cooldown_move_counter: Res<CooldownMoveCounter>,
    mut turn_text_query: Query<&mut Text, (With<TurnText>, Without<ScoreText>, Without<LevelText>)>,
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
            "Ходов: {}/{} ({})",
            turn_counter.0, spawn_rows_left.0, cooldown_move_counter.value
        );
    }
    for mut level_text in &mut level_text_query {
        level_text.sections[0].value = format!("Уровень: ");
    }
}
