use bevy::prelude::{Commands, Input, KeyCode, NextState, Res, ResMut};

use crate::{
    components::AppState,
    loading::font_assets::FontAssets,
    ui::{
        components::{NextStateButton, NoneComponent},
        resources::{ColorType, UIMenuButtonColors, UIMenuTextColors},
        utils::{
            button_utils::{append_large_text_button, append_middle_text_button},
            camera_utils::build_ui_camera,
            menu_utils::build_menu,
            text_utils::append_large_text,
        },
    },
};

pub fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<UIMenuButtonColors>,
    text_colors: Res<UIMenuTextColors>,
) {
    build_ui_camera(&mut commands);
    build_menu(&mut commands, |parent| {
        append_large_text(
            parent,
            "Игра окончена",
            &font_assets,
            &text_colors,
            None::<NoneComponent>,
        );
        append_large_text_button(
            parent,
            Some(NextStateButton {
                color_type: ColorType::Blue,
                next_state: AppState::GameplayInit,
            }),
            &ColorType::Blue,
            "Заново",
            &font_assets,
            &text_colors,
            &button_colors,
            false,
        );
        append_middle_text_button(
            parent,
            Some(NextStateButton {
                color_type: ColorType::Gray,
                next_state: AppState::StartMenu,
            }),
            &ColorType::Gray,
            "Главное меню",
            &font_assets,
            &text_colors,
            &button_colors,
            false,
        );
    });
}

pub fn keydown_detect(
    mut app_state_next_state: ResMut<NextState<AppState>>,
    keyboard_input_key_code: Res<Input<KeyCode>>,
) {
    if keyboard_input_key_code.any_just_released([KeyCode::Escape]) {
        app_state_next_state.set(AppState::StartMenu);
    }
    if keyboard_input_key_code.any_just_released([KeyCode::Space]) {
        app_state_next_state.set(AppState::GameplayInit);
    }
}
