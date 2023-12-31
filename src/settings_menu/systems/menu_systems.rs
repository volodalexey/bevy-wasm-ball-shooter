use bevy::prelude::{Commands, Res};
use bevy_pkv::PkvStore;

use crate::{
    components::AppState,
    constants::{
        INIT_ROWS_KEY, MOVE_DOWN_AFTER_KEY, TOTAL_COLORS_KEY, TOTAL_COLUMNS_KEY, TOTAL_ROWS_KEY,
    },
    game_audio::constants::{MAIN_SOUND_VOLUME_KEY, SFX_SOUND_VOLUME_KEY},
    loading::font_assets::FontAssets,
    settings_menu::utils::{
        colors_utils::build_colors_line, columns_utils::build_columns_line,
        move_down_utils::build_move_down_line, rows_utils::build_rows_line,
        volume_utils::build_volume_line,
    },
    ui::{
        components::{NextStateButton, NoneComponent},
        resources::{ColorType, UIMenuButtonColors, UIMenuTextColors},
        utils::{
            button_utils::append_middle_text_button, camera_utils::build_ui_camera,
            menu_utils::build_menu, text_utils::append_large_text,
        },
    },
};

pub fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<UIMenuButtonColors>,
    text_colors: Res<UIMenuTextColors>,
    pkv: Res<PkvStore>,
) {
    build_ui_camera(&mut commands);
    build_menu(&mut commands, |parent| {
        append_large_text(
            parent,
            "Настройки",
            &font_assets,
            &text_colors,
            None::<NoneComponent>,
        );
        build_volume_line(
            "Фоновый звук",
            MAIN_SOUND_VOLUME_KEY,
            parent,
            &font_assets,
            &button_colors,
            &text_colors,
            &pkv,
        );
        build_volume_line(
            "Звук выстрела/очков",
            SFX_SOUND_VOLUME_KEY,
            parent,
            &font_assets,
            &button_colors,
            &text_colors,
            &pkv,
        );
        build_colors_line(
            "Всего цветов",
            TOTAL_COLORS_KEY,
            parent,
            &font_assets,
            &button_colors,
            &text_colors,
            &pkv,
        );
        build_columns_line(
            "Число шаров в ряду",
            TOTAL_COLUMNS_KEY,
            parent,
            &font_assets,
            &button_colors,
            &text_colors,
            &pkv,
        );
        build_rows_line(
            "Рядов вначале",
            INIT_ROWS_KEY,
            "Всего рядов",
            TOTAL_ROWS_KEY,
            parent,
            &font_assets,
            &button_colors,
            &text_colors,
            &pkv,
        );
        build_move_down_line(
            "Новый ряд после",
            MOVE_DOWN_AFTER_KEY,
            parent,
            &font_assets,
            &button_colors,
            &text_colors,
            &pkv,
        );
        append_middle_text_button(
            parent,
            Some(NextStateButton {
                color_type: ColorType::Gray,
                next_state: AppState::StartMenu,
            }),
            &ColorType::Gray,
            "Назад",
            &font_assets,
            &text_colors,
            &button_colors,
            false,
        );
    });
}
