use bevy::{
    prelude::{default, Bundle, ChildBuilder, Color, Res, TextBundle},
    text::{Text, TextSection, TextStyle},
};

use crate::{
    loading::font_assets::FontAssets,
    ui::{
        components::ResponsiveText,
        constants::{LARGE_FONT_SIZE, MIDDLE_FONT_SIZE},
        resources::UIMenuTextColors,
    },
};

use super::responsive_utils::is_mobile;

pub fn build_sized_text(
    text: &str,
    font_size: f32,
    font_assets: &Res<FontAssets>,
    text_color: &Color,
) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: text.to_string(),
                style: TextStyle {
                    font: font_assets.fira_sans_bold.clone_weak(),
                    font_size,
                    color: (*text_color).into(),
                },
            }],
            ..default()
        },
        ..default()
    }
}

pub fn build_large_text(
    text: &str,
    font_assets: &Res<FontAssets>,
    text_colors: &Res<UIMenuTextColors>,
) -> TextBundle {
    build_sized_text(text, LARGE_FONT_SIZE, font_assets, &text_colors.title)
}

pub fn build_middle_text(
    text: &str,
    font_assets: &Res<FontAssets>,
    text_colors: &Res<UIMenuTextColors>,
) -> TextBundle {
    build_sized_text(text, MIDDLE_FONT_SIZE, font_assets, &text_colors.title)
}

pub fn append_sized_text(
    parent: &mut ChildBuilder<'_, '_, '_>,
    text: &str,
    font_size: f32,
    font_assets: &Res<FontAssets>,
    text_color: &Color,
    some_bundle: Option<impl Bundle>,
) {
    if let Some(bundle) = some_bundle {
        parent.spawn((
            build_sized_text(text, font_size, font_assets, text_color),
            bundle,
        ));
    } else {
        parent.spawn(build_sized_text(text, font_size, font_assets, text_color));
    }
}

pub fn append_large_text(
    parent: &mut ChildBuilder<'_, '_, '_>,
    text: &str,
    font_assets: &Res<FontAssets>,
    text_colors: &Res<UIMenuTextColors>,
    some_bundle: Option<impl Bundle>,
) {
    append_sized_text(
        parent,
        text,
        LARGE_FONT_SIZE,
        font_assets,
        &text_colors.title,
        some_bundle,
    )
}

pub fn append_middle_text(
    parent: &mut ChildBuilder<'_, '_, '_>,
    text: &str,
    font_assets: &Res<FontAssets>,
    text_colors: &Res<UIMenuTextColors>,
    some_bundle: Option<impl Bundle>,
) {
    append_sized_text(
        parent,
        text,
        MIDDLE_FONT_SIZE,
        font_assets,
        &text_colors.title,
        some_bundle,
    )
}

pub fn build_responsive_text(
    window_width: f32,
    text: &str,
    font_assets: &Res<FontAssets>,
    text_colors: &Res<UIMenuTextColors>,
) -> TextBundle {
    if is_mobile(window_width) {
        build_middle_text(text, font_assets, text_colors)
    } else {
        build_large_text(text, font_assets, text_colors)
    }
}

pub fn append_responsive_text(
    parent: &mut ChildBuilder<'_, '_, '_>,
    window_width: f32,
    text: &str,
    font_assets: &Res<FontAssets>,
    text_colors: &Res<UIMenuTextColors>,
    some_bundle: Option<impl Bundle>,
) {
    if let Some(bundle) = some_bundle {
        parent.spawn((
            build_responsive_text(window_width, text, font_assets, text_colors),
            ResponsiveText {},
            bundle,
        ));
    } else {
        parent.spawn((
            build_responsive_text(window_width, text, font_assets, text_colors),
            ResponsiveText {},
        ));
    }
}
