use bevy::prelude::{ChildBuilder, Res};
use bevy_pkv::PkvStore;

use crate::{
    loading::font_assets::FontAssets,
    settings_menu::components::VolumeButton,
    ui::{
        resources::{ColorType, UIMenuButtonColors, UIMenuTextColors},
        utils::{
            append_flex_row_evenly, append_middle_text_button, build_flex_column_start,
            build_middle_text,
        },
    },
};

pub fn build_volume_line(
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
            [0.0, 0.01, 0.1, 0.3, 0.5, 1.0].map(|v| {
                let selected = match pkv.get::<String>(key) {
                    Ok(sound_volume) => {
                        if let Ok(parsed) = sound_volume.parse::<f32>() {
                            parsed == v
                        } else {
                            false
                        }
                    }
                    Err(_) => false,
                };
                append_middle_text_button(
                    parent,
                    VolumeButton {
                        value: v,
                        key: key.to_string(),
                        pressed: selected,
                        color_type: ColorType::Green,
                    },
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
