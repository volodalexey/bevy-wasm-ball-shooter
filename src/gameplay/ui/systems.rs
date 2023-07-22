use bevy::{
    prelude::{
        default, BuildChildren, Commands, DespawnRecursiveExt, Entity, NodeBundle, Query, Res,
        TextBundle, With, Without,
    },
    text::{Text, TextSection},
    ui::{AlignItems, Display, FlexDirection, JustifyContent, Style, Val},
};

use crate::{gameplay::ui::components::StatusBar, loading::font_assets::FontAssets};

use super::{
    components::{LevelText, ScoreText, TurnText},
    resources::{LevelCounter, ScoreCounter, TurnCounter},
    utils::get_text_style,
};

pub fn setup_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::FlexStart,
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    ..default()
                },
                ..default()
            },
            StatusBar {},
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "".to_string(),
                            style: get_text_style(&font_assets),
                        }],
                        ..default()
                    },
                    ..default()
                },
                ScoreText {},
            ));
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "".to_string(),
                            style: get_text_style(&font_assets),
                        }],
                        ..default()
                    },
                    ..default()
                },
                TurnText {},
            ));
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "".to_string(),
                            style: get_text_style(&font_assets),
                        }],
                        ..default()
                    },
                    ..default()
                },
                LevelText {},
            ));
        });
}

pub fn update_ui(
    score_counter: Res<ScoreCounter>,
    mut score_text_query: Query<
        &mut Text,
        (With<ScoreText>, Without<TurnText>, Without<LevelText>),
    >,
    turn_counter: Res<TurnCounter>,
    mut turn_text_query: Query<&mut Text, (With<TurnText>, Without<ScoreText>, Without<LevelText>)>,
    level_counter: Res<LevelCounter>,
    mut level_text_query: Query<
        &mut Text,
        (With<LevelText>, Without<ScoreText>, Without<TurnText>),
    >,
) {
    for mut score_text in &mut score_text_query {
        score_text.sections[0].value = format!("Очки: {:?} ", score_counter.0);
    }
    for mut turn_text in &mut turn_text_query {
        turn_text.sections[0].value = format!("Ходов: {}", turn_counter.0);
    }
    for mut level_text in &mut level_text_query {
        level_text.sections[0].value = format!("Уровень: {}", level_counter.0);
    }
}

pub fn cleanup_ui(mut commands: Commands, query: Query<Entity, With<StatusBar>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
