use bevy::{
    prelude::{
        default, BuildChildren, Bundle, ButtonBundle, ChildBuilder, Color, ImageBundle, Res,
    },
    ui::{AlignItems, Interaction, JustifyContent, Style, UiRect, Val},
};

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
use crate::ui::components::QuitButton;

use crate::{
    loading::{font_assets::FontAssets, sprite_assets::SpriteAssets},
    ui::{
        constants::{
            LARGE_BUTTON_FONT_SIZE, LARGE_BUTTON_ICON_WIDTH, LARGE_BUTTON_PADDING,
            MIDDLE_BUTTON_FONT_SIZE, MIDDLE_BUTTON_ICON_WIDTH, MIDDLE_BUTTON_PADDING,
        },
        resources::{ColorType, UIMenuButtonColors, UIMenuTextColors},
    },
};

use super::text_utils::build_sized_text;

pub fn append_large_text_button(
    parent: &mut ChildBuilder<'_, '_, '_>,
    some_bundle: Option<impl Bundle>,
    color_type: &ColorType,
    text: &str,
    font_assets: &Res<FontAssets>,
    text_colors: &Res<UIMenuTextColors>,
    button_colors: &Res<UIMenuButtonColors>,
    selected: bool,
) {
    append_size_text_button(
        UiRect::all(Val::Px(LARGE_BUTTON_PADDING)),
        LARGE_BUTTON_FONT_SIZE,
        parent,
        some_bundle,
        color_type,
        text,
        &text_colors.primary_button,
        font_assets,
        button_colors,
        selected,
    )
}

pub fn append_middle_text_button(
    parent: &mut ChildBuilder<'_, '_, '_>,
    some_bundle: Option<impl Bundle>,
    color_type: &ColorType,
    text: &str,
    font_assets: &Res<FontAssets>,
    text_colors: &Res<UIMenuTextColors>,
    button_colors: &Res<UIMenuButtonColors>,
    selected: bool,
) {
    append_size_text_button(
        UiRect::all(Val::Px(MIDDLE_BUTTON_PADDING)),
        MIDDLE_BUTTON_FONT_SIZE,
        parent,
        some_bundle,
        color_type,
        text,
        &text_colors.primary_button,
        font_assets,
        button_colors,
        selected,
    )
}

pub fn build_button_bundle(
    padding: UiRect,
    selected: bool,
    button_colors: &Res<UIMenuButtonColors>,
    color_type: &ColorType,
) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            padding,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color: button_color_by_interaction(
            selected,
            button_colors,
            color_type,
            &Interaction::None,
        )
        .into(),
        ..Default::default()
    }
}

pub fn append_size_text_button(
    padding: UiRect,
    font_size: f32,
    parent: &mut ChildBuilder<'_, '_, '_>,
    some_bundle: Option<impl Bundle>,
    color_type: &ColorType,
    text: &str,
    text_color: &Color,
    font_assets: &Res<FontAssets>,
    button_colors: &Res<UIMenuButtonColors>,
    selected: bool,
) {
    if let Some(bundle) = some_bundle {
        parent
            .spawn((
                build_button_bundle(padding, selected, button_colors, color_type),
                bundle,
            ))
            .with_children(|parent| {
                parent.spawn(build_sized_text(text, font_size, font_assets, text_color));
            });
    } else {
        parent
            .spawn(build_button_bundle(
                padding,
                selected,
                button_colors,
                color_type,
            ))
            .with_children(|parent| {
                parent.spawn(build_sized_text(text, font_size, font_assets, text_color));
            });
    }
}

pub fn append_size_icon_button(
    padding: UiRect,
    parent: &mut ChildBuilder<'_, '_, '_>,
    bundle: impl Bundle,
    color_type: &ColorType,
    sprite_assets: &Res<SpriteAssets>,
    button_colors: &Res<UIMenuButtonColors>,
    selected: bool,
    width: f32,
) {
    parent
        .spawn((
            build_button_bundle(padding, selected, button_colors, color_type),
            bundle,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: sprite_assets.arrow_up_right_from_square.clone().into(),
                style: Style {
                    width: Val::Px(width),
                    ..default()
                },
                ..default()
            });
        });
}

#[allow(dead_code)]
pub fn append_large_icon_button(
    parent: &mut ChildBuilder<'_, '_, '_>,
    bundle: impl Bundle,
    color_type: &ColorType,
    sprite_assets: &Res<SpriteAssets>,
    button_colors: &Res<UIMenuButtonColors>,
    selected: bool,
) {
    append_size_icon_button(
        UiRect::all(Val::Px(LARGE_BUTTON_PADDING)),
        parent,
        bundle,
        color_type,
        sprite_assets,
        button_colors,
        selected,
        LARGE_BUTTON_ICON_WIDTH,
    )
}

pub fn append_middle_icon_button(
    parent: &mut ChildBuilder<'_, '_, '_>,
    bundle: impl Bundle,
    color_type: &ColorType,
    sprite_assets: &Res<SpriteAssets>,
    button_colors: &Res<UIMenuButtonColors>,
    selected: bool,
) {
    append_size_icon_button(
        UiRect::all(Val::Px(MIDDLE_BUTTON_PADDING)),
        parent,
        bundle,
        color_type,
        sprite_assets,
        button_colors,
        selected,
        MIDDLE_BUTTON_ICON_WIDTH,
    )
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub fn build_quit_button(
    parent: &mut ChildBuilder<'_, '_, '_>,
    font_assets: &Res<FontAssets>,
    text_colors: &Res<UIMenuTextColors>,
    button_colors: &Res<UIMenuButtonColors>,
) {
    append_middle_text_button(
        parent,
        Some(QuitButton {
            color_type: ColorType::Gray,
        }),
        &ColorType::Gray,
        "Выход",
        font_assets,
        text_colors,
        button_colors,
        false,
    )
}

pub fn button_color_by_interaction(
    selected: bool,
    button_colors: &Res<UIMenuButtonColors>,
    color_type: &ColorType,
    interaction: &Interaction,
) -> Color {
    match *interaction {
        Interaction::Pressed => match color_type {
            ColorType::Gray => button_colors.gray_pressed,
            ColorType::Green => button_colors.green_pressed,
            ColorType::Blue => button_colors.blue_pressed,
        },
        Interaction::Hovered => match selected {
            true => match color_type {
                ColorType::Gray => button_colors.gray_selected_hovered,
                ColorType::Green => button_colors.green_selected_hovered,
                ColorType::Blue => button_colors.blue_selected_hovered,
            },
            false => match color_type {
                ColorType::Gray => button_colors.gray_hovered,
                ColorType::Green => button_colors.green_hovered,
                ColorType::Blue => button_colors.blue_hovered,
            },
        },
        Interaction::None => match selected {
            true => match color_type {
                ColorType::Gray => button_colors.gray_selected,
                ColorType::Green => button_colors.green_selected,
                ColorType::Blue => button_colors.blue_selected,
            },
            false => match color_type {
                ColorType::Gray => button_colors.gray_idle,
                ColorType::Green => button_colors.green_idle,
                ColorType::Blue => button_colors.blue_idle,
            },
        },
    }
}
