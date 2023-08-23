use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::{
        default, Color, Commands, DespawnRecursiveExt, Entity, Input, KeyCode, Query, Res, ResMut,
        TextBundle, With,
    },
    text::{Text, TextStyle, DEFAULT_FONT_HANDLE},
    ui::{PositionType, Style, Val},
};
use bevy_xpbd_2d::prelude::PhysicsLoop;

use super::components::FpsText;

pub fn control_physics(mut physics_loop: ResMut<PhysicsLoop>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::P) {
        if physics_loop.paused {
            physics_loop.resume();
        } else {
            physics_loop.pause();
        }
    }
    if keys.just_pressed(KeyCode::O) && physics_loop.paused {
        physics_loop.step();
    }
}

pub fn setup_fps_text(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "FPS: ",
            TextStyle {
                font: DEFAULT_FONT_HANDLE.typed(),
                font_size: 20.0,
                color: Color::TOMATO,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        FpsText,
    ));
}

pub fn cleanup_fps_text(mut commands: Commands, fps_text_query: Query<Entity, With<FpsText>>) {
    for text_entity in fps_text_query.iter() {
        commands.entity(text_entity).despawn_recursive();
    }
}

pub fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_text_query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut fps_text_query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[0].value = format!("FPS: {value:.2}");
            }
        }
    }
}
