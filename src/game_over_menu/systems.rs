use bevy::{
    prelude::{
        default, BuildChildren, Button, ButtonBundle, Camera2dBundle, Changed, Color, Commands,
        DespawnRecursiveExt, Entity, NextState, NodeBundle, Query, Res, ResMut, TextBundle, With,
    },
    text::{Text, TextSection, TextStyle},
    ui::{AlignItems, BackgroundColor, Interaction, JustifyContent, Size, Style, UiRect, Val},
};

use crate::{components::AppState, loading::font_assets::FontAssets};

use super::{
    components::{GameOverMenu, GameOverMenuCamera},
    resources::ButtonColors,
};

pub fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
) {
    commands.spawn((Camera2dBundle::default(), GameOverMenuCamera {}));
    commands
        .spawn((NodeBundle::default(), GameOverMenu {}))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Game over".to_string(),
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
                        size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                        margin: UiRect::all(Val::Auto),
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
                                value: "Play".to_string(),
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
    button_colors: Res<ButtonColors>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
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
    camera_query: Query<Entity, With<GameOverMenuCamera>>,
    node_query: Query<Entity, With<GameOverMenu>>,
) {
    commands.entity(camera_query.single()).despawn_recursive();
    commands.entity(node_query.single()).despawn_recursive();
}
