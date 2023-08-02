use bevy::{
    prelude::{
        AudioSink, ChildBuilder, Commands, Input, KeyCode, NextState, Query, Res, ResMut, With,
    },
    ui::{BackgroundColor, Interaction},
};
use bevy_pkv::PkvStore;

use crate::{
    components::AppState,
    constants::LEVEL_KEY,
    game_audio::{
        components::MainSound,
        constants::{MAIN_SOUND_VOLUME_KEY, SFX_SOUND_VOLUME_KEY},
        utils::{play_score_audio, play_shoot_audio, toggle_main_audio},
    },
    gameplay::constants::{MAX_LEVEL, START_LEVEL},
    loading::{audio_assets::AudioAssets, font_assets::FontAssets},
    resources::LevelCounter,
    ui::{
        components::NextStateButton,
        resources::{ColorType, UIMenuButtonColors, UIMenuTextColors},
        utils::{
            build_flex_column_start, build_flex_column_stretch, build_flex_row_between,
            build_flex_row_evenly, build_large_text, build_menu, build_middle_button,
            build_middle_text, build_ui_camera, button_color_by_interaction,
        },
    },
};

use super::components::{LevelButton, VolumeButton};

pub fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<UIMenuButtonColors>,
    text_colors: Res<UIMenuTextColors>,
    pkv: Res<PkvStore>,
) {
    build_ui_camera(&mut commands);
    build_menu(&mut commands, |parent| {
        build_large_text(parent, "Настройки", &font_assets, &text_colors);
        build_volume_row(
            "Фоновый звук",
            MAIN_SOUND_VOLUME_KEY,
            parent,
            &font_assets,
            &button_colors,
            &text_colors,
            &pkv,
        );
        build_volume_row(
            "Звук выстрела/очков",
            SFX_SOUND_VOLUME_KEY,
            parent,
            &font_assets,
            &button_colors,
            &text_colors,
            &pkv,
        );
        build_level_settings(
            "Уровень",
            LEVEL_KEY,
            parent,
            &font_assets,
            &button_colors,
            &text_colors,
            &pkv,
        );
        build_middle_button(
            parent,
            NextStateButton {
                color_type: ColorType::Gray,
                next_state: AppState::StartMenu,
            },
            &ColorType::Gray,
            "Назад",
            &font_assets,
            &text_colors,
            &button_colors,
            false,
        );
    });
}

pub fn build_volume_row(
    title: &str,
    key: &str,
    parent: &mut ChildBuilder<'_, '_, '_>,
    font_assets: &Res<FontAssets>,
    button_colors: &Res<UIMenuButtonColors>,
    text_colors: &Res<UIMenuTextColors>,
    pkv: &Res<PkvStore>,
) {
    build_flex_column_start(parent, |parent| {
        build_middle_text(parent, title, font_assets, text_colors);
        build_flex_row_evenly(parent, |parent| {
            [0.0, 0.01, 0.1, 0.3, 0.5, 1.0].map(|v| {
                let selected = match pkv.get::<String>(key) {
                    Ok(sound_volume) => {
                        if let Ok(parsed) = sound_volume.parse::<f32>() {
                            parsed == v
                        } else {
                            false
                        }
                    }
                    Err(_) => false,
                };
                build_middle_button(
                    parent,
                    VolumeButton {
                        value: v,
                        key: key.to_string(),
                        pressed: selected,
                        color_type: ColorType::Green,
                    },
                    &ColorType::Green,
                    v.to_string().as_str(),
                    font_assets,
                    text_colors,
                    button_colors,
                    selected,
                );
            });
        });
    });
}

pub fn interact_with_volume_button(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    main_sound_query: Query<&AudioSink, With<MainSound>>,
    button_colors: Res<UIMenuButtonColors>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut VolumeButton),
        With<VolumeButton>,
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
    for (idx, (interaction, mut background_color, mut button_volume)) in
        button_query.iter_mut().enumerate()
    {
        match *interaction {
            Interaction::Pressed => {
                if !button_volume.pressed {
                    button_volume.pressed = true;
                    pkv.set_string(button_volume.key.clone(), &button_volume.value.to_string())
                        .expect("failed to save volume");
                    *background_color = button_color_by_interaction(
                        button_volume.pressed,
                        &button_colors,
                        &button_volume.color_type,
                        interaction,
                    )
                    .into();

                    match button_volume.key.as_str() {
                        MAIN_SOUND_VOLUME_KEY => {
                            toggle_main_audio(&main_sound_query, button_volume.value);
                        }
                        SFX_SOUND_VOLUME_KEY => {
                            if fastrand::bool() {
                                play_shoot_audio(&mut commands, &audio_assets, button_volume.value);
                            } else {
                                play_score_audio(&mut commands, &audio_assets, button_volume.value);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Interaction::Hovered => {
                *background_color = button_color_by_interaction(
                    button_volume.pressed,
                    &button_colors,
                    &button_volume.color_type,
                    interaction,
                )
                .into();
            }
            Interaction::None => {
                if pressed_button.0 > -1
                    && pressed_button.0 != idx as i32
                    && pressed_button.1 == button_volume.key
                {
                    if button_volume.pressed {
                        button_volume.pressed = false;
                    }
                }
                *background_color = button_color_by_interaction(
                    button_volume.pressed,
                    &button_colors,
                    &button_volume.color_type,
                    interaction,
                )
                .into();
            }
        };
    }
}

pub fn keydown_detect(
    mut app_state_next_state: ResMut<NextState<AppState>>,
    keyboard_input_key_code: Res<Input<KeyCode>>,
) {
    if keyboard_input_key_code.any_just_released([KeyCode::Escape]) {
        app_state_next_state.set(AppState::StartMenu);
    }
}

pub fn build_level_button(
    saved_level: u32,
    iter_level: u32,
    parent: &mut ChildBuilder<'_, '_, '_>,
    button_colors: &Res<UIMenuButtonColors>,
    font_assets: &Res<FontAssets>,
    text_colors: &Res<UIMenuTextColors>,
) {
    let selected = saved_level == iter_level;
    build_middle_button(
        parent,
        LevelButton {
            level: iter_level,
            pressed: selected,
            color_type: ColorType::Green,
        },
        &ColorType::Green,
        iter_level.to_string().as_str(),
        font_assets,
        text_colors,
        button_colors,
        selected,
    );
}

pub fn build_level_line(
    saved_level: u32,
    range: std::ops::RangeInclusive<u32>,
    parent: &mut ChildBuilder<'_, '_, '_>,
    button_colors: &Res<UIMenuButtonColors>,
    font_assets: &Res<FontAssets>,
    text_colors: &Res<UIMenuTextColors>,
) {
    build_flex_row_between(parent, |parent| {
        range.for_each(|l| {
            build_level_button(
                saved_level,
                l,
                parent,
                button_colors,
                font_assets,
                text_colors,
            );
        });
    });
}

pub fn build_level_settings(
    title: &str,
    key: &str,
    parent: &mut ChildBuilder<'_, '_, '_>,
    font_assets: &Res<FontAssets>,
    button_colors: &Res<UIMenuButtonColors>,
    text_colors: &Res<UIMenuTextColors>,
    pkv: &Res<PkvStore>,
) {
    build_flex_column_start(parent, |parent| {
        build_middle_text(parent, title, font_assets, text_colors);

        build_flex_column_stretch(parent, |parent| {
            let saved_level = match pkv.get::<String>(key) {
                Ok(level) => {
                    if let Ok(level) = level.parse::<u32>() {
                        level
                    } else {
                        START_LEVEL
                    }
                }
                Err(_) => START_LEVEL,
            };

            build_level_line(
                saved_level,
                START_LEVEL..=9,
                parent,
                button_colors,
                font_assets,
                text_colors,
            );
            build_level_line(
                saved_level,
                10..=MAX_LEVEL,
                parent,
                button_colors,
                font_assets,
                text_colors,
            );
        });
    });
}

pub fn interact_with_level_button(
    mut commands: Commands,
    button_colors: Res<UIMenuButtonColors>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut LevelButton),
        With<LevelButton>,
    >,
    mut pkv: ResMut<PkvStore>,
) {
    let pressed_button_idx: i32 = match button_query
        .iter()
        .enumerate()
        .find(|(_, (interaction, _, _))| **interaction == Interaction::Pressed)
    {
        Some((idx, _)) => idx as i32,
        None => -1,
    };
    for (idx, (interaction, mut background_color, mut button_level)) in
        button_query.iter_mut().enumerate()
    {
        match *interaction {
            Interaction::Pressed => {
                if !button_level.pressed {
                    button_level.pressed = true;
                    pkv.set_string(LEVEL_KEY, &button_level.level.to_string())
                        .expect("failed to save level");
                    *background_color = button_color_by_interaction(
                        button_level.pressed,
                        &button_colors,
                        &button_level.color_type,
                        interaction,
                    )
                    .into();

                    commands.insert_resource(LevelCounter(button_level.level));
                }
            }
            Interaction::Hovered => {
                *background_color = button_color_by_interaction(
                    button_level.pressed,
                    &button_colors,
                    &button_level.color_type,
                    interaction,
                )
                .into();
            }
            Interaction::None => {
                if pressed_button_idx > -1 && pressed_button_idx != idx as i32 {
                    if button_level.pressed {
                        button_level.pressed = false;
                    }
                }
                *background_color = button_color_by_interaction(
                    button_level.pressed,
                    &button_colors,
                    &button_level.color_type,
                    interaction,
                )
                .into();
            }
        };
    }
}
