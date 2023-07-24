use bevy::{
    prelude::{
        default, BuildChildren, ButtonBundle, Camera2dBundle, ChildBuilder, Color, Commands,
        DespawnRecursiveExt, Entity, Input, KeyCode, NextState, NodeBundle, Query, Res, ResMut,
        TextBundle, With,
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
    game_audio::constants::{MAIN_SOUND_VOLUME_KEY, SHOOT_SOUND_VOLUME_KEY},
    loading::font_assets::FontAssets,
};

use super::{
    components::{SettingsMenu, SettingsMenuCamera, VolumeButton},
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
                "Звук выстрела",
                SHOOT_SOUND_VOLUME_KEY,
                parent,
                &font_assets,
                &button_colors,
                &pkv,
            );
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
        app_state_next_state.set(AppState::GameplayInit);
    }
}
