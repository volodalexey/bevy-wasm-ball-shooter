use bevy::prelude::{ChildBuilder, Res};
use bevy_pkv::PkvStore;

use crate::{
    constants::DEFAULT_ROWS_COUNT,
    loading::font_assets::FontAssets,
    settings_menu::components::{TotalRowsButton, TotalRowsText},
    ui::{
        components::NoneComponent,
        resources::{ColorType, UIMenuButtonColors, UIMenuTextColors},
        utils::{
            button_utils::append_middle_text_button,
            flex_utils::{append_flex_row_evenly, build_flex_column_start},
            text_utils::{append_large_text, append_middle_text},
        },
    },
};

pub fn read_total_rows(key: &str, pkv: &PkvStore) -> u8 {
    match pkv.get::<String>(key) {
        Ok(total_rows) => {
            if let Ok(parsed) = total_rows.parse::<u8>() {
                parsed
            } else {
                DEFAULT_ROWS_COUNT
            }
        }
        Err(_) => DEFAULT_ROWS_COUNT,
    }
}

pub fn build_rows_line(
    title: &str,
    key: &str,
    parent: &mut ChildBuilder<'_, '_, '_>,
    font_assets: &Res<FontAssets>,
    button_colors: &Res<UIMenuButtonColors>,
    text_colors: &Res<UIMenuTextColors>,
    pkv: &Res<PkvStore>,
) {
    build_flex_column_start(parent, |parent| {
        append_middle_text(
            parent,
            title,
            font_assets,
            text_colors,
            None::<NoneComponent>,
        );
        append_flex_row_evenly(parent, |parent| {
            append_middle_text_button(
                parent,
                Some(TotalRowsButton {
                    increment: false,
                    key: key.to_string(),
                    color_type: ColorType::Green,
                    pressed: false,
                }),
                &ColorType::Green,
                "-",
                font_assets,
                text_colors,
                button_colors,
                false,
            );
            let total_rows = read_total_rows(key, pkv);
            append_large_text(
                parent,
                total_rows.to_string().as_str(),
                font_assets,
                text_colors,
                Some(TotalRowsText {}),
            );
            append_middle_text_button(
                parent,
                Some(TotalRowsButton {
                    increment: true,
                    key: key.to_string(),
                    color_type: ColorType::Green,
                    pressed: false,
                }),
                &ColorType::Green,
                "+",
                font_assets,
                text_colors,
                button_colors,
                false,
            );
        });
    });
}
