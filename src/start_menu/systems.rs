use bevy::prelude::{Commands, Input, KeyCode, NextState, Res, ResMut};
#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
use bevy::{app::AppExit, prelude::EventWriter};

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
use crate::ui::utils::build_quit_button;
use crate::{
    components::AppState,
    loading::font_assets::FontAssets,
    ui::{
        components::NextStateButton,
        resources::{ColorType, UIMenuButtonColors, UIMenuTextColors},
        utils::{
            append_large_text_button, append_middle_text_button, build_large_text, build_menu,
            build_ui_camera,
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
        build_large_text(parent, "Шарики веселяшки", &font_assets, &text_colors);
        append_large_text_button(
            parent,
            NextStateButton {
                color_type: ColorType::Green,
                next_state: AppState::GameplayInit,
            },
            &ColorType::Green,
            "Играть",
            &font_assets,
            &text_colors,
            &button_colors,
            false,
        );
        append_middle_text_button(
            parent,
            NextStateButton {
                color_type: ColorType::Gray,
                next_state: AppState::Settings,
            },
            &ColorType::Gray,
            "Настройки",
            &font_assets,
            &text_colors,
            &button_colors,
            false,
        );
        #[cfg(not(target_arch = "wasm32"))]
        #[allow(dead_code)]
        build_quit_button(parent, &font_assets, &text_colors, &button_colors);
    });
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub fn keydown_quit_detect(
    mut app_exit_event_writer: EventWriter<AppExit>,
    keyboard_input_key_code: Res<Input<KeyCode>>,
) {
    if keyboard_input_key_code.any_just_released([KeyCode::Escape]) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn keydown_init_detect(
    mut app_state_next_state: ResMut<NextState<AppState>>,
    keyboard_input_key_code: Res<Input<KeyCode>>,
) {
    if keyboard_input_key_code.any_just_released([KeyCode::Space]) {
        app_state_next_state.set(AppState::GameplayInit);
    }
}
