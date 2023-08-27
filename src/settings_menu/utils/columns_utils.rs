use bevy::prelude::{ChildBuilder, Res};
use bevy_pkv::PkvStore;

use crate::{
    constants::{DEFAULT_COLUMNS_COUNT, MAX_COLUMNS_COUNT, MIN_COLUMNS_COUNT},
    loading::font_assets::FontAssets,
    settings_menu::components::TotalColumnsButton,
    ui::{
        components::NoneComponent,
        resources::{ColorType, UIMenuButtonColors, UIMenuTextColors},
        utils::{
            button_utils::append_middle_text_button,
            flex_utils::{append_flex_row_evenly, build_flex_column_start},
            text_utils::append_middle_text,
        },
    },
};

pub fn read_init_cols(key: &str, pkv: &Res<PkvStore>) -> u8 {
    match pkv.get::<String>(key) {
        Ok(init_cols) => {
            if let Ok(parsed) = init_cols.parse::<u8>() {
                parsed
            } else {
                DEFAULT_COLUMNS_COUNT
            }
        }
        Err(_) => DEFAULT_COLUMNS_COUNT,
    }
}

pub fn build_columns_line(
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
            let init_cols = read_init_cols(key, pkv);
            (MIN_COLUMNS_COUNT..=MAX_COLUMNS_COUNT).for_each(|v| {
                let selected = init_cols == v;
                append_middle_text_button(
                    parent,
                    Some(TotalColumnsButton {
                        value: v,
                        key: key.to_string(),
                        pressed: selected,
                        color_type: ColorType::Green,
                    }),
                    &ColorType::Green,
                    format!("{}/{}", v, v + 1).as_str(),
                    font_assets,
                    text_colors,
                    button_colors,
                    selected,
                );
            });
        });
    });
}
