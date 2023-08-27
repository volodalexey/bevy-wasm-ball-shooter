use bevy::{
    prelude::{Query, Res, ResMut, With},
    ui::{BackgroundColor, Interaction},
};
use bevy_pkv::PkvStore;

use crate::{
    settings_menu::components::TotalColumnsButton,
    ui::{resources::UIMenuButtonColors, utils::button_utils::button_color_by_interaction},
};

pub fn interact_with_columns_button(
    button_colors: Res<UIMenuButtonColors>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut TotalColumnsButton),
        With<TotalColumnsButton>,
    >,
    mut pkv: ResMut<PkvStore>,
) {
    let pressed_button: (i32, String) = match button_query
        .iter()
        .enumerate()
        .find(|(_, (interaction, _, _))| **interaction == Interaction::Pressed)
    {
        Some((idx, (_, _, button_volume))) => (idx as i32, button_volume.key.clone()),
        None => (-1, "".to_string()),
    };
    for (idx, (interaction, mut background_color, mut columns_button)) in
        button_query.iter_mut().enumerate()
    {
        match *interaction {
            Interaction::Pressed => {
                if !columns_button.pressed {
                    columns_button.pressed = true;
                    pkv.set_string(
                        columns_button.key.clone(),
                        &columns_button.value.to_string(),
                    )
                    .expect("failed to save total colors");
                    *background_color = button_color_by_interaction(
                        columns_button.pressed,
                        &button_colors,
                        &columns_button.color_type,
                        interaction,
                    )
                    .into();
                }
            }
            Interaction::Hovered => {
                *background_color = button_color_by_interaction(
                    columns_button.pressed,
                    &button_colors,
                    &columns_button.color_type,
                    interaction,
                )
                .into();
            }
            Interaction::None => {
                if pressed_button.0 > -1
                    && pressed_button.0 != idx as i32
                    && pressed_button.1 == columns_button.key
                {
                    if columns_button.pressed {
                        columns_button.pressed = false;
                    }
                }
                *background_color = button_color_by_interaction(
                    columns_button.pressed,
                    &button_colors,
                    &columns_button.color_type,
                    interaction,
                )
                .into();
            }
        };
    }
}
