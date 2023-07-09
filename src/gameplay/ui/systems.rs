use bevy::{
    prelude::{
        default, BuildChildren, Commands, DespawnRecursiveExt, Entity, NodeBundle, Query, Res,
        TextBundle, With, Without,
    },
    text::{Text, TextSection},
    ui::{AlignItems, Display, FlexDirection, JustifyContent, Size, Style, Val},
};

use crate::{
    gameplay::{
        constants::MOVE_DOWN_TURN,
        resources::{RoundTurnCounter, Score},
        ui::components::StatusBar,
    },
    loading::font_assets::FontAssets,
};

use super::{
    components::{ScoreText, TurnText},
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
                    size: Size::new(Val::Percent(100.0), Val::Auto),
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
        });
}

pub fn update_ui(
    score: Res<Score>,
    mut score_text_query: Query<&mut Text, (With<ScoreText>, Without<TurnText>)>,
    round_turn_counter: Res<RoundTurnCounter>,
    mut turn_text_query: Query<&mut Text, (With<TurnText>, Without<ScoreText>)>,
) {
    for mut score_text in &mut score_text_query {
        score_text.sections[0].value = format!("Score: {:?} ", score.0);
    }
    for mut turn_text in &mut turn_text_query {
        turn_text.sections[0].value = format!("Turn: {}/{}", round_turn_counter.0, MOVE_DOWN_TURN);
    }
}

pub fn cleanup_ui(mut commands: Commands, query: Query<Entity, With<StatusBar>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
