use bevy::{
    prelude::{Color, Res},
    text::TextStyle,
};

use crate::loading::font_assets::FontAssets;

pub fn get_text_style(font_assets: &Res<FontAssets>) -> TextStyle {
    TextStyle {
        font: font_assets.fira_sans_bold.clone_weak(),
        font_size: 40.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    }
}
