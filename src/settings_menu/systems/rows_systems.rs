use bevy::{
    prelude::{Input, MouseButton, Query, Res, ResMut, Touches, With},
    text::Text,
    ui::{BackgroundColor, Interaction},
};
use bevy_pkv::PkvStore;

use crate::{
    constants::{MAX_ROWS_COUNT, MIN_ROWS_COUNT, TOTAL_ROWS_KEY},
    settings_menu::{
        components::{TotalRowsButton, TotalRowsText},
        utils::rows_utils::read_total_rows,
    },
    ui::{resources::UIMenuButtonColors, utils::button_utils::button_color_by_interaction},
};

pub fn interact_with_rows_button(
    button_colors: Res<UIMenuButtonColors>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut TotalRowsButton),
        With<TotalRowsButton>,
    >,
    mut pkv: ResMut<PkvStore>,
    mouse_button_input: Res<Input<MouseButton>>,
    touches_input: Res<Touches>,
) {
    let is_mouse_up = mouse_button_input.just_released(MouseButton::Left);
    let is_touch_released = touches_input.any_just_released();
    for (interaction, mut background_color, mut rows_button) in button_query.iter_mut() {
        if is_mouse_up || is_touch_released {
            rows_button.pressed = false;
        }
        match *interaction {
            Interaction::Pressed => {
                if !rows_button.pressed {
                    rows_button.pressed = true;
                    let mut total_rows = read_total_rows(&rows_button.key, pkv.as_ref()) as i32
                        + match rows_button.increment {
                            true => 1,
                            false => -1,
                        };
                    if total_rows < MIN_ROWS_COUNT as i32 {
                        total_rows = MIN_ROWS_COUNT as i32
                    } else if total_rows > MAX_ROWS_COUNT as i32 {
                        total_rows = MAX_ROWS_COUNT as i32
                    }

                    pkv.set_string(rows_button.key.clone(), &total_rows.to_string())
                        .expect("failed to save total colors");
                    *background_color = button_color_by_interaction(
                        rows_button.pressed,
                        &button_colors,
                        &rows_button.color_type,
                        interaction,
                    )
                    .into();
                }
            }
            Interaction::Hovered => {
                *background_color = button_color_by_interaction(
                    rows_button.pressed,
                    &button_colors,
                    &rows_button.color_type,
                    interaction,
                )
                .into();
            }
            Interaction::None => {
                if rows_button.pressed {
                    rows_button.pressed = false;
                }
                *background_color = button_color_by_interaction(
                    rows_button.pressed,
                    &button_colors,
                    &rows_button.color_type,
                    interaction,
                )
                .into();
            }
        };
    }
}

pub fn update_total_rows_text(
    pkv: Res<PkvStore>,
    mut rows_text_query: Query<&mut Text, With<TotalRowsText>>,
) {
    for mut rows_text in &mut rows_text_query {
        let total_rows = read_total_rows(&TOTAL_ROWS_KEY, pkv.as_ref());
        rows_text.sections[0].value = format!(" {:?} ", total_rows);
    }
}
