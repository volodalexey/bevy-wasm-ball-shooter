use bevy::prelude::{ChildBuilder, Res};
use bevy_pkv::PkvStore;

use crate::{
    constants::{MAX_COLUMNS_COUNT, MIN_COLUMNS_COUNT},
    loading::font_assets::FontAssets,
    settings_menu::components::TotalColumnsButton,
    ui::{
        resources::{ColorType, UIMenuButtonColors, UIMenuTextColors},
        utils::{
            append_flex_row_evenly, append_middle_text_button, build_flex_column_start,
            build_middle_text,
        },
    },
};

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
        build_middle_text(parent, title, font_assets, text_colors);
        append_flex_row_evenly(parent, |parent| {
            (MIN_COLUMNS_COUNT..=MAX_COLUMNS_COUNT).for_each(|v| {
                let selected = match pkv.get::<String>(key) {
                    Ok(colors_count) => {
                        if let Ok(parsed) = colors_count.parse::<u8>() {
                            parsed == v
                        } else {
                            false
                        }
                    }
                    Err(_) => false,
                };
                append_middle_text_button(
                    parent,
                    TotalColumnsButton {
                        value: v,
                        key: key.to_string(),
                        pressed: selected,
                        color_type: ColorType::Green,
                    },
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
