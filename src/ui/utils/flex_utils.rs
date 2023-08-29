use bevy::{
    prelude::{default, BuildChildren, ChildBuilder, Commands, NodeBundle},
    ui::{AlignItems, Display, FlexDirection, JustifyContent, Style, Val},
};

use crate::ui::{
    components::UIFullRow,
    constants::{COLUMN_ROW_GAP, ROW_COLUMN_GAP},
};

fn build_flex_col(align_items: AlignItems) -> NodeBundle {
    NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(COLUMN_ROW_GAP),
            justify_content: JustifyContent::Center,
            align_items,
            ..default()
        },
        ..default()
    }
}

fn append_flex_column(
    parent: &mut ChildBuilder<'_, '_, '_>,
    children: impl FnOnce(&mut ChildBuilder),
    align_items: AlignItems,
) {
    parent
        .spawn(build_flex_col(align_items))
        .with_children(children);
}

pub fn append_flex_column_start(
    parent: &mut ChildBuilder<'_, '_, '_>,
    children: impl FnOnce(&mut ChildBuilder),
) {
    append_flex_column(parent, children, AlignItems::Start)
}
#[allow(dead_code)]
pub fn spawn_flex_column_stretch(
    parent: &mut ChildBuilder<'_, '_, '_>,
    children: impl FnOnce(&mut ChildBuilder),
) {
    append_flex_column(parent, children, AlignItems::Stretch)
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
#[allow(dead_code)]
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
