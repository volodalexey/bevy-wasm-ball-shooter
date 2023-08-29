use bevy::prelude::{ChildBuilder, Res};
use bevy_pkv::PkvStore;

use crate::{
    constants::DEFAULT_MOVE_DOWN_AFTER,
    loading::font_assets::FontAssets,
    settings_menu::components::MoveDownButton,
    ui::{
        components::NoneComponent,
        resources::{ColorType, UIMenuButtonColors, UIMenuTextColors},
        utils::{
            button_utils::append_middle_text_button,
            flex_utils::{append_flex_column_start, append_flex_row_evenly},
            text_utils::append_middle_text,
        },
    },
};

pub fn read_move_down(key: &str, pkv: &Res<PkvStore>) -> u8 {
    match pkv.get::<String>(key) {
        Ok(colors_count) => {
            if let Ok(parsed) = colors_count.parse::<u8>() {
                parsed
            } else {
                DEFAULT_MOVE_DOWN_AFTER
            }
        }
        Err(_) => DEFAULT_MOVE_DOWN_AFTER,
    }
}

pub fn build_move_down_line(
    title: &str,
    key: &str,
    parent: &mut ChildBuilder<'_, '_, '_>,
    font_assets: &Res<FontAssets>,
    button_colors: &Res<UIMenuButtonColors>,
    text_colors: &Res<UIMenuTextColors>,
    pkv: &Res<PkvStore>,
) {
    append_flex_column_start(parent, |parent| {
        append_middle_text(
            parent,
            title,
            font_assets,
            text_colors,
            None::<NoneComponent>,
        );
        append_flex_row_evenly(parent, |parent| {
            let move_down = read_move_down(key, pkv);
            [1, 2, 3, 5, 10].iter().for_each(|v| {
                let selected = move_down == *v;
                append_middle_text_button(
                    parent,
                    Some(MoveDownButton {
                        value: *v,
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
