#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
use bevy::{app::AppExit, prelude::EventWriter};
use bevy::{
    prelude::{
        Changed, Commands, DespawnRecursiveExt, Entity, EventReader, NextState, Query, Res, ResMut,
        With,
    },
    text::Text,
    time::Time,
    ui::{BackgroundColor, Interaction},
    window::WindowResized,
};

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
use super::components::QuitButton;
use super::{
    components::{NextStateButton, ResponsiveText, UICamera, UIFullRow, UIMenu},
    constants::{LARGE_FONT_SIZE, MIDDLE_FONT_SIZE},
    resources::{PointerCooldown, UIMenuButtonColors},
    utils::{button_color_by_interaction, is_mobile},
};
use crate::components::AppState;

pub fn tick_pointer_cooldown_timer(mut pointer_cooldown: ResMut<PointerCooldown>, time: Res<Time>) {
    if pointer_cooldown.started {
        pointer_cooldown.timer.tick(time.delta());
        if pointer_cooldown.timer.finished() {
            pointer_cooldown.started = false;
        }
    }
}

pub fn cleanup_menu(
    mut commands: Commands,
    camera_query: Query<Entity, With<UICamera>>,
    menu_query: Query<Entity, With<UIMenu>>,
) {
    for camera_entity in camera_query.iter() {
        commands.entity(camera_entity).despawn_recursive();
    }
    for menu_entity in menu_query.iter() {
        commands.entity(menu_entity).despawn_recursive();
    }
}

pub fn cleanup_full_row(mut commands: Commands, rows_query: Query<Entity, With<UIFullRow>>) {
    for entity in rows_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn interact_with_next_state_button(
    button_colors: Res<UIMenuButtonColors>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &NextStateButton),
        (Changed<Interaction>, With<NextStateButton>),
    >,
    mut pointer_cooldown: ResMut<PointerCooldown>,
) {
    for (interaction, mut background_color, next_state_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = button_color_by_interaction(
                    false,
                    &button_colors,
                    &next_state_button.color_type,
                    interaction,
                )
                .into();
                pointer_cooldown.started = true;
                app_state_next_state.set(next_state_button.next_state);
            }
            Interaction::Hovered | Interaction::None => {
                *background_color = button_color_by_interaction(
                    false,
                    &button_colors,
                    &next_state_button.color_type,
                    interaction,
                )
                .into();
            }
        }
    }
}
#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &QuitButton),
        (Changed<Interaction>, With<QuitButton>),
    >,
    button_colors: Res<UIMenuButtonColors>,
) {
    if let Ok((interaction, mut background_color, quit_button)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = button_color_by_interaction(
                    false,
                    &button_colors,
                    &quit_button.color_type,
                    interaction,
                )
                .into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered | Interaction::None => {
                *background_color = button_color_by_interaction(
                    false,
                    &button_colors,
                    &quit_button.color_type,
                    interaction,
                )
                .into();
            }
        }
    }
}

pub fn resize_responsive_text(
    mut resize_reader: EventReader<WindowResized>,
    mut text_query: Query<&mut Text, With<ResponsiveText>>,
) {
    if let Some(e) = resize_reader.iter().next() {
        let font_size = match is_mobile(e.width) {
            true => MIDDLE_FONT_SIZE,
            false => LARGE_FONT_SIZE,
        };
        for mut text in &mut text_query {
            for mut section in text.sections.iter_mut() {
                section.style.font_size = font_size;
            }
        }
        resize_reader.clear();
    }
}
