use bevy::prelude::{ChildBuilder, Res};
use bevy_pkv::PkvStore;

use crate::{
    constants::{DEFAULT_COLORS_COUNT, MAX_COLORS_COUNT, MIN_COLORS_COUNT},
    loading::font_assets::FontAssets,
    settings_menu::components::TotalColorsButton,
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

pub fn read_total_colors(key: &str, pkv: &Res<PkvStore>) -> u8 {
    match pkv.get::<String>(key) {
        Ok(colors_count) => {
            if let Ok(parsed) = colors_count.parse::<u8>() {
                parsed
            } else {
                DEFAULT_COLORS_COUNT
            }
        }
        Err(_) => DEFAULT_COLORS_COUNT,
    }
}

pub fn build_colors_line(
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
            let total_colors = read_total_colors(key, pkv);
            (MIN_COLORS_COUNT..=MAX_COLORS_COUNT).for_each(|v| {
                let selected = total_colors == v;
                append_middle_text_button(
                    parent,
                    Some(TotalColorsButton {
                        value: v,
                        key: key.to_string(),
                        pressed: selected,
                        color_type: ColorType::Green,
                    }),
                    &ColorType::Green,
                    v.to_string().as_str(),
                    font_assets,
                    text_colors,
                    button_colors,
                    selected,
                );
            });
        });
    });
}
