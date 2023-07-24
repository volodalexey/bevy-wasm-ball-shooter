use bevy::{
    prelude::{
        default, AudioSink, BuildChildren, ButtonBundle, Camera2dBundle, Changed, ChildBuilder,
        Color, Commands, DespawnRecursiveExt, Entity, Input, KeyCode, NextState, NodeBundle, Query,
        Res, ResMut, TextBundle, With,
    },
    text::{Text, TextSection, TextStyle},
    ui::{
        AlignItems, BackgroundColor, Display, FlexDirection, Interaction, JustifyContent, Style,
        UiRect, Val,
    },
};
use bevy_pkv::PkvStore;

use crate::{
    components::AppState,
    game_audio::{
        components::MainSound,
        constants::{MAIN_SOUND_VOLUME_KEY, SFX_SOUND_VOLUME_KEY},
        utils::{play_score_audio, play_shoot_audio, toggle_main_audio},
    },
    gameplay::constants::{LEVEL_KEY, MAX_LEVEL, START_LEVEL},
    loading::{audio_assets::AudioAssets, font_assets::FontAssets},
    resources::LevelCounter,
};

use super::{
    components::{BackButton, LevelButton, SettingsMenu, SettingsMenuCamera, VolumeButton},
    resources::SettingsButtonColors,
};

pub fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<SettingsButtonColors>,
    pkv: Res<PkvStore>,
) {
    commands.spawn((Camera2dBundle::default(), SettingsMenuCamera {}));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    row_gap: Val::Px(10.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            SettingsMenu {},
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Настройки".to_string(),
                        style: TextStyle {
                            font: font_assets.fira_sans_bold.clone_weak(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    ..default()
                },
                ..default()
            });

            build_volume_row(
                "Фоновый звук",
                MAIN_SOUND_VOLUME_KEY,
                parent,
                &font_assets,
                &button_colors,
                &pkv,
            );
            build_volume_row(
                "Звук выстрела/очков",
                SFX_SOUND_VOLUME_KEY,
                parent,
                &font_assets,
                &button_colors,
                &pkv,
            );
            build_level_settings(
                "Уровень",
                LEVEL_KEY,
                parent,
                &font_assets,
                &button_colors,
                &pkv,
            );

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: button_colors.normal.into(),
                        ..Default::default()
                    },
                    BackButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Назад".to_string(),
                                style: TextStyle {
                                    font: font_assets.fira_sans_bold.clone_weak(),
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            }],
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}

pub fn build_volume_row(
    title: &str,
    key: &str,
    parent: &mut ChildBuilder<'_, '_, '_>,
    font_assets: &Res<FontAssets>,
    button_colors: &Res<SettingsButtonColors>,
    pkv: &Res<PkvStore>,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: title.to_string(),
                        style: TextStyle {
                            font: font_assets.fira_sans_bold.clone_weak(),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    ..default()
                },
                ..default()
            });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(6.0),
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    [0.0, 0.01, 0.1, 0.3, 0.5, 1.0].map(|v| {
                        let pressed = match pkv.get::<String>(key) {
                            Ok(sound_volume) => {
                                if let Ok(parsed) = sound_volume.parse::<f32>() {
                                    parsed == v
                                } else {
                                    false
                                }
                            }
                            Err(_) => false,
                        };
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        padding: UiRect::all(Val::Px(10.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    background_color: match pressed {
                                        true => button_colors.pressed.into(),
                                        false => button_colors.normal.into(),
                                    },
                                    ..Default::default()
                                },
                                VolumeButton {
                                    value: v,
                                    key: key.to_string(),
                                    pressed,
                                },
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    text: Text {
                                        sections: vec![TextSection {
                                            value: v.to_string(),
                                            style: TextStyle {
                                                font: font_assets.fira_sans_bold.clone_weak(),
                                                font_size: 20.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        }],
                                        ..default()
                                    },
                                    ..default()
                                });
                            });
                    });
                });
        });
}

pub fn interact_with_volume_button(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    main_sound_query: Query<&AudioSink, With<MainSound>>,
    button_colors: Res<SettingsButtonColors>,
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
    for (idx, (interaction, mut button_color, mut button_volume)) in
        button_query.iter_mut().enumerate()
    {
        match *interaction {
            Interaction::Pressed => {
                if !button_volume.pressed {
                    button_volume.pressed = true;
                    pkv.set_string(button_volume.key.clone(), &button_volume.value.to_string())
                        .expect("failed to save volume");
                    *button_color = button_colors.pressed.into();

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
                if button_volume.pressed {
                    *button_color = button_colors.pressed_hovered.into();
                } else {
                    *button_color = button_colors.normal_hovered.into();
                }
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
                if button_volume.pressed {
                    *button_color = button_colors.pressed.into();
                } else {
                    *button_color = button_colors.normal.into();
                }
            }
        };
    }
}

pub fn interact_with_back_button(
    button_colors: Res<SettingsButtonColors>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BackButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = button_colors.back_pressed.into();
                app_state_next_state.set(AppState::StartMenu);
            }
            Interaction::Hovered => {
                *color = button_colors.back_hovered.into();
            }
            Interaction::None => {
                *color = button_colors.back_idle.into();
            }
        }
    }
}

pub fn cleanup_menu(
    mut commands: Commands,
    camera_query: Query<Entity, With<SettingsMenuCamera>>,
    node_query: Query<Entity, With<SettingsMenu>>,
) {
    commands.entity(camera_query.single()).despawn_recursive();
    commands.entity(node_query.single()).despawn_recursive();
}

pub fn keydown_detect(
    mut app_state_next_state: ResMut<NextState<AppState>>,
    keyboard_input_key_code: Res<Input<KeyCode>>,
) {
    if keyboard_input_key_code.any_pressed([KeyCode::Space]) {
        app_state_next_state.set(AppState::StartMenu);
    }
}

pub fn build_level_button(
    saved_level: u32,
    iter_level: u32,
    parent: &mut ChildBuilder<'_, '_, '_>,
    button_colors: &Res<SettingsButtonColors>,
    font_assets: &Res<FontAssets>,
) {
    let pressed = saved_level == iter_level;
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: match pressed {
                    true => button_colors.pressed.into(),
                    false => button_colors.normal.into(),
                },
                ..Default::default()
            },
            LevelButton {
                level: iter_level,
                pressed,
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: iter_level.to_string(),
                        style: TextStyle {
                            font: font_assets.fira_sans_bold.clone_weak(),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    ..default()
                },
                ..default()
            });
        });
}

pub fn build_level_line(
    saved_level: u32,
    range: std::ops::RangeInclusive<u32>,
    parent: &mut ChildBuilder<'_, '_, '_>,
    button_colors: &Res<SettingsButtonColors>,
    font_assets: &Res<FontAssets>,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(6.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            range.for_each(|l| {
                build_level_button(saved_level, l, parent, button_colors, font_assets);
            });
        });
}

pub fn build_level_settings(
    title: &str,
    key: &str,
    parent: &mut ChildBuilder<'_, '_, '_>,
    font_assets: &Res<FontAssets>,
    button_colors: &Res<SettingsButtonColors>,
    pkv: &Res<PkvStore>,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: title.to_string(),
                        style: TextStyle {
                            font: font_assets.fira_sans_bold.clone_weak(),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    ..default()
                },
                ..default()
            });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(6.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Stretch,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
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
                    );
                    build_level_line(
                        saved_level,
                        10..=MAX_LEVEL,
                        parent,
                        button_colors,
                        font_assets,
                    );
                });
        });
}

pub fn interact_with_level_button(
    mut commands: Commands,
    button_colors: Res<SettingsButtonColors>,
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
    for (idx, (interaction, mut button_color, mut button_level)) in
        button_query.iter_mut().enumerate()
    {
        match *interaction {
            Interaction::Pressed => {
                if !button_level.pressed {
                    button_level.pressed = true;
                    pkv.set_string(LEVEL_KEY, &button_level.level.to_string())
                        .expect("failed to save level");
                    *button_color = button_colors.pressed.into();

                    commands.insert_resource(LevelCounter(button_level.level));
                }
            }
            Interaction::Hovered => {
                if button_level.pressed {
                    *button_color = button_colors.pressed_hovered.into();
                } else {
                    *button_color = button_colors.normal_hovered.into();
                }
            }
            Interaction::None => {
                if pressed_button_idx > -1 && pressed_button_idx != idx as i32 {
                    if button_level.pressed {
                        button_level.pressed = false;
                    }
                }
                if button_level.pressed {
                    *button_color = button_colors.pressed.into();
                } else {
                    *button_color = button_colors.normal.into();
                }
            }
        };
    }
}
