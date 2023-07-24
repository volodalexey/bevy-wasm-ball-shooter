use bevy::{
    prelude::{
        default, BuildChildren, Button, ButtonBundle, Camera2dBundle, Changed, Color, Commands,
        DespawnRecursiveExt, Entity, Input, KeyCode, NextState, NodeBundle, Query, Res, ResMut,
        TextBundle, With,
    },
    text::{Text, TextSection, TextStyle},
    ui::{
        AlignItems, BackgroundColor, Display, FlexDirection, Interaction, JustifyContent, Style,
        UiRect, Val,
    },
};

use crate::{
    components::AppState,
    gameplay::constants::{MAX_LEVEL, START_LEVEL},
    loading::font_assets::FontAssets,
    resources::{LevelCounter, PointerCooldown},
};

use super::{
    components::{GameWinMenu, GameWinMenuCamera},
    resources::GameWinButtonColors,
};

pub fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<GameWinButtonColors>,
) {
    commands.spawn((Camera2dBundle::default(), GameWinMenuCamera {}));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
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
            GameWinMenu {},
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Поздравляем!".to_string(),
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
                                value: "Далее".to_string(),
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

pub fn click_play_button(
    button_colors: Res<GameWinButtonColors>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut pointer_cooldown: ResMut<PointerCooldown>,
    mut level_counter: ResMut<LevelCounter>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                pointer_cooldown.started = true;
                level_counter.0 += 1;
                if level_counter.0 > MAX_LEVEL {
                    level_counter.0 = START_LEVEL
                }
                app_state_next_state.set(AppState::GameplayInit);
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
    camera_query: Query<Entity, With<GameWinMenuCamera>>,
    node_query: Query<Entity, With<GameWinMenu>>,
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
