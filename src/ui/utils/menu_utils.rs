use bevy::{
    prelude::{default, BuildChildren, ChildBuilder, Commands, NodeBundle},
    ui::{AlignItems, FlexDirection, JustifyContent, Style, Val},
};

use crate::ui::{components::UIMenu, constants::MENU_ROW_GAP};

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
