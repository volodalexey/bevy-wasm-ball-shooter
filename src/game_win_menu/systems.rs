use bevy::prelude::{Commands, Input, KeyCode, NextState, Res, ResMut};

use crate::{
    components::AppState,
    loading::font_assets::FontAssets,
    ui::{
        components::NextStateButton,
        resources::{ColorType, UIMenuButtonColors, UIMenuTextColors},
        utils::{
            build_large_button, build_large_text, build_menu, build_middle_button, build_ui_camera,
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
        build_large_text(parent, "Поздравляем!", &font_assets, &text_colors);
        build_large_button(
            parent,
            NextStateButton {
                color_type: ColorType::Green,
                next_state: AppState::GameplayInit,
            },
            &ColorType::Green,
            "Далее",
            &font_assets,
            &text_colors,
            &button_colors,
            false,
        );
        build_middle_button(
            parent,
            NextStateButton {
                color_type: ColorType::Gray,
                next_state: AppState::StartMenu,
            },
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
