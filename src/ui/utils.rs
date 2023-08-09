use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{
        default, BuildChildren, Bundle, ButtonBundle, Camera2d, Camera2dBundle, ChildBuilder,
        Color, Commands, Component, ImageBundle, NodeBundle, Res, TextBundle,
    },
    text::{Text, TextSection, TextStyle},
    ui::{AlignItems, Display, FlexDirection, Interaction, JustifyContent, Style, UiRect, Val},
};

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
use super::components::QuitButton;
use super::{
    components::{NoneComponent, UICamera, UIFullRow, UIMenu},
    constants::{
        COLUMN_ROW_GAP, LARGE_BUTTON_FONT_SIZE, LARGE_BUTTON_ICON_WIDTH, LARGE_BUTTON_PADDING,
        LARGE_FONT_SIZE, MENU_ROW_GAP, MIDDLE_BUTTON_FONT_SIZE, MIDDLE_BUTTON_ICON_WIDTH,
        MIDDLE_BUTTON_PADDING, MIDDLE_FONT_SIZE, ROW_COLUMN_GAP,
    },
    resources::{ColorType, UIMenuButtonColors, UIMenuTextColors},
};
use crate::loading::font_assets::FontAssets;
use crate::loading::sprite_assets::SpriteAssets;

pub fn build_ui_camera(commands: &mut Commands) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            ..default()
        },
        UICamera {},
    ));
}

pub fn build_menu(commands: &mut Commands, children: impl FnOnce(&mut ChildBuilder)) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    row_gap: Val::Px(MENU_ROW_GAP),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            UIMenu {},
        ))
        .with_children(children);
}

pub fn build_large_text(
    parent: &mut ChildBuilder<'_, '_, '_>,
    text: &str,
    font_assets: &Res<FontAssets>,
    text_colors: &Res<UIMenuTextColors>,
) {
    build_sized_text(
        parent,
        text,
        LARGE_FONT_SIZE,
        font_assets,
        &text_colors.title,
        NoneComponent {},
    );
}

pub fn build_large_text_component(
    parent: &mut ChildBuilder<'_, '_, '_>,
    text: &str,
    font_assets: &Res<FontAssets>,
    text_colors: &Res<UIMenuTextColors>,
    bundle: impl Bundle,
) {
    build_sized_text(
        parent,
        text,
        LARGE_FONT_SIZE,
        font_assets,
        &text_colors.title,
        bundle,
    );
}

pub fn build_middle_text(
    parent: &mut ChildBuilder<'_, '_, '_>,
    text: &str,
    font_assets: &Res<FontAssets>,
    text_colors: &Res<UIMenuTextColors>,
) {
    build_sized_text(
        parent,
        text,
        MIDDLE_FONT_SIZE,
        font_assets,
        &text_colors.title,
        NoneComponent {},
    );
}

pub fn build_middle_text_component(
    parent: &mut ChildBuilder<'_, '_, '_>,
    text: &str,
    font_assets: &Res<FontAssets>,
    text_colors: &Res<UIMenuTextColors>,
    bundle: impl Bundle,
) {
    build_sized_text(
        parent,
        text,
        MIDDLE_FONT_SIZE,
        font_assets,
        &text_colors.title,
        bundle,
    );
}

pub fn is_mobile(window_width: f32) -> bool {
    window_width < 700.0
}

pub fn build_responsive_text_component(
    window_width: f32,
    parent: &mut ChildBuilder<'_, '_, '_>,
    text: &str,
    font_assets: &Res<FontAssets>,
    text_colors: &Res<UIMenuTextColors>,
    bundle: impl Bundle,
) {
    if is_mobile(window_width) {
        build_middle_text_component(parent, text, font_assets, text_colors, bundle)
    } else {
        build_large_text_component(parent, text, font_assets, text_colors, bundle)
    }
}

fn build_sized_text(
    parent: &mut ChildBuilder<'_, '_, '_>,
    text: &str,
    font_size: f32,
    font_assets: &Res<FontAssets>,
    text_color: &Color,
    bundle: impl Bundle,
) {
    let mut spawned = parent.spawn(TextBundle {
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
    });
    spawned.insert(bundle);
}

pub fn append_large_text_button(
    parent: &mut ChildBuilder<'_, '_, '_>,
    component: impl Component,
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
        component,
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
    component: impl Component,
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
        component,
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
    component: impl Component,
    color_type: &ColorType,
    text: &str,
    text_color: &Color,
    font_assets: &Res<FontAssets>,
    button_colors: &Res<UIMenuButtonColors>,
    selected: bool,
) {
    parent
        .spawn((
            build_button_bundle(padding, selected, button_colors, color_type),
            component,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
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
            });
        });
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
        QuitButton {
            color_type: ColorType::Gray,
        },
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

fn build_flex_column(
    parent: &mut ChildBuilder<'_, '_, '_>,
    children: impl FnOnce(&mut ChildBuilder),
    align_items: AlignItems,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(COLUMN_ROW_GAP),
                justify_content: JustifyContent::Center,
                align_items,
                ..default()
            },
            ..default()
        })
        .with_children(children);
}

pub fn build_flex_column_start(
    parent: &mut ChildBuilder<'_, '_, '_>,
    children: impl FnOnce(&mut ChildBuilder),
) {
    build_flex_column(parent, children, AlignItems::Start)
}

pub fn build_flex_column_stretch(
    parent: &mut ChildBuilder<'_, '_, '_>,
    children: impl FnOnce(&mut ChildBuilder),
) {
    build_flex_column(parent, children, AlignItems::Stretch)
}

fn build_flex_row(
    justify_content: JustifyContent,
    align_items: AlignItems,
    width: Val,
    height: Val,
) -> NodeBundle {
    NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(ROW_COLUMN_GAP),
            justify_content,
            align_items,
            width,
            height,
            ..default()
        },
        ..default()
    }
}

pub fn append_flex_row(
    parent: &mut ChildBuilder<'_, '_, '_>,
    children: impl FnOnce(&mut ChildBuilder),
    justify_content: JustifyContent,
    align_items: AlignItems,
    width: Val,
    height: Val,
) {
    parent
        .spawn(build_flex_row(justify_content, align_items, width, height))
        .with_children(children);
}

pub fn append_flex_row_evenly(
    parent: &mut ChildBuilder<'_, '_, '_>,
    children: impl FnOnce(&mut ChildBuilder),
) {
    append_flex_row(
        parent,
        children,
        JustifyContent::SpaceEvenly,
        AlignItems::Center,
        Val::Auto,
        Val::Auto,
    )
}

pub fn build_flex_full_row_evenly(
    commands: &mut Commands,
    children: impl FnOnce(&mut ChildBuilder),
) {
    commands
        .spawn((
            build_flex_row(
                JustifyContent::SpaceEvenly,
                AlignItems::FlexStart,
                Val::Percent(100.0),
                Val::Auto,
            ),
            UIFullRow {},
        ))
        .with_children(children);
}

pub fn append_flex_row_between(
    parent: &mut ChildBuilder<'_, '_, '_>,
    children: impl FnOnce(&mut ChildBuilder),
) {
    append_flex_row(
        parent,
        children,
        JustifyContent::SpaceBetween,
        AlignItems::Center,
        Val::Auto,
        Val::Auto,
    )
}
