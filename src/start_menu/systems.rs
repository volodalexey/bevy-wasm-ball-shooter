use bevy::{
    audio::{Volume, VolumeLevel},
    prelude::{
        default, AudioBundle, BuildChildren, Button, ButtonBundle, Camera2dBundle, Changed, Color,
        Commands, DespawnRecursiveExt, Entity, Input, KeyCode, NextState, NodeBundle,
        PlaybackSettings, Query, Res, ResMut, TextBundle, With,
    },
    text::{Text, TextSection, TextStyle},
    ui::{
        AlignItems, BackgroundColor, FlexDirection, Interaction, JustifyContent, Style, UiRect, Val,
    },
};

use crate::{
    components::AppState,
    loading::{audio_assets::AudioAssets, font_assets::FontAssets},
    resources::PointerCooldown,
};

use super::{
    components::{MainSoundtrack, StartMenu, StartMenuCamera},
    resources::ButtonColors,
};

pub fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
) {
    commands.spawn((Camera2dBundle::default(), StartMenuCamera {}));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    column_gap: Val::Px(10.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            StartMenu {},
        ))
        .with_children(|parent| {
            let text_style = TextStyle {
                font: font_assets.fira_sans_bold.clone_weak(),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            };
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Шарики".to_string(),
                            style: text_style.clone(),
                        },
                        TextSection {
                            value: " ".to_string(),
                            style: text_style.clone(),
                        },
                        TextSection {
                            value: "веселяшки".to_string(),
                            style: text_style,
                        },
                    ],
                    ..default()
                },
                ..default()
            });
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: button_colors.normal.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Играть".to_string(),
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
                });
        });
}

pub fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    commands.spawn((
        AudioBundle {
            source: audio_assets.soundtrack.clone_weak(),
            settings: PlaybackSettings::LOOP.with_volume(Volume::Relative(VolumeLevel::new(0.1))),
            ..default()
        },
        MainSoundtrack {},
    ));
}

pub fn click_play_button(
    button_colors: Res<ButtonColors>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut pointer_cooldown: ResMut<PointerCooldown>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                pointer_cooldown.started = true;
                app_state_next_state.set(AppState::Gameplay);
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

pub fn cleanup_menu(
    mut commands: Commands,
    camera_query: Query<Entity, With<StartMenuCamera>>,
    node_query: Query<Entity, With<StartMenu>>,
) {
    commands.entity(camera_query.single()).despawn_recursive();
    commands.entity(node_query.single()).despawn_recursive();
}

pub fn cleanup_audio(
    mut commands: Commands,
    soundtrack_query: Query<Entity, With<MainSoundtrack>>,
) {
    commands
        .entity(soundtrack_query.single())
        .despawn_recursive();
}

pub fn keydown_detect(
    mut app_state_next_state: ResMut<NextState<AppState>>,
    keyboard_input_key_code: Res<Input<KeyCode>>,
) {
    if keyboard_input_key_code.any_pressed([KeyCode::Space]) {
        app_state_next_state.set(AppState::Gameplay);
    }
}
