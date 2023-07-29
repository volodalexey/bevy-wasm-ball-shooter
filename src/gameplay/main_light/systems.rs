use bevy::prelude::{
    default, Color, Commands, DespawnRecursiveExt, DirectionalLight, DirectionalLightBundle,
    Entity, Query, Transform, Vec3, With,
};

use crate::gameplay::constants::PROJECTILE_SPAWN;

use super::components::MainLight;

pub fn setup_main_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 25000.0,
                color: Color::WHITE,
                ..default()
            },
            transform: Transform::from_xyz(10.0, 70.0, 41.0)
                .looking_at(Vec3::new(0.0, 0.0, PROJECTILE_SPAWN / 2.), Vec3::Y),
            ..default()
        },
        MainLight,
    ));
}

pub fn cleanup_main_light(mut commands: Commands, query: Query<Entity, With<MainLight>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
