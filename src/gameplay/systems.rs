use bevy::prelude::{
    default, Assets, AudioBundle, Color, Commands, Entity, EventReader, EventWriter, Mesh,
    NextState, PlaybackSettings, Query, Res, ResMut, StandardMaterial, Transform, Vec3, With,
};
use bevy_prototype_debug_lines::DebugLines;
use hexx::{Direction, Hex};

use crate::{
    components::AppState,
    gameplay::{
        ball::BallBundle,
        constants::MOVE_DOWN_TURN,
        grid::{
            components::HexComponent,
            events::MoveDownAndSpawn,
            utils::{find_cluster, find_floating_clusters},
        },
        projectile::utils::clamp_inside_world_bounds,
    },
    loading::audio_assets::AudioAssets,
};

use super::{
    ball::{Ball, Species},
    constants::PLAYER_SPAWN_Z,
    events::BeginTurn,
    grid::resources::Grid,
    projectile::{
        components::{Flying, Projectile},
        events::SnapProjectile,
    },
    resources::{RoundTurnCounter, Score, TurnCounter},
};

pub fn setup_gameplay(
    mut begin_turn: EventWriter<BeginTurn>,
    mut turn_counter: ResMut<TurnCounter>,
    mut round_turn_counter: ResMut<RoundTurnCounter>,
    mut score: ResMut<Score>,
) {
    score.0 = 0;
    turn_counter.0 = 0;
    round_turn_counter.0 = 0;
    begin_turn.send(BeginTurn);
}

pub fn on_begin_turn(
    mut turn_counter: ResMut<TurnCounter>,
    mut round_turn_counter: ResMut<RoundTurnCounter>,
    mut begin_turn: EventReader<BeginTurn>,
) {
    if begin_turn.is_empty() {
        return;
    }
    begin_turn.clear();
    turn_counter.0 += 1;
    round_turn_counter.0 += 1;
}

pub fn check_game_over(
    grid: Res<Grid>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut lines: ResMut<DebugLines>,
) {
    let projectile_hex = grid.layout.world_pos_to_hex(hexx::Vec2 {
        x: 0.0,
        y: PLAYER_SPAWN_Z,
    });
    let game_over_row = projectile_hex
        .neighbor(Direction::Top)
        .neighbor(Direction::Top);

    let (_, z) = grid
        .layout
        .hex_to_world_pos(Hex::new(0, game_over_row.y))
        .into();

    lines.line_colored(
        Vec3::new(grid.bounds.mins.x, 0., z),
        Vec3::new(grid.bounds.maxs.x, 0., z),
        0.,
        Color::RED,
    );

    for (&hex, _) in grid.storage.iter() {
        let world_pos = grid.layout.hex_to_world_pos(hex);
        if world_pos.y >= z - 0.1 {
            app_state_next_state.set(AppState::GameOver);
            break;
        }
    }
}

pub fn on_snap_projectile(
    mut snap_projectile: EventReader<SnapProjectile>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut grid: ResMut<Grid>,
    mut begin_turn: EventWriter<BeginTurn>,
    mut score: ResMut<Score>,
    turn_counter: ResMut<TurnCounter>,
    mut round_turn_counter: ResMut<RoundTurnCounter>,
    projectile: Query<(Entity, &Transform, &Species), (With<Projectile>, With<Flying>)>,
    balls: Query<&Species, With<Ball>>,
    audio_assets: Res<AudioAssets>,
    mut move_down_and_spawn: EventWriter<MoveDownAndSpawn>,
) {
    if snap_projectile.is_empty() {
        return;
    }

    // We really only care about the first ball hit event
    snap_projectile.clear();

    if let Ok((entity, tr, species)) = projectile.get_single() {
        commands.entity(entity).despawn();

        let mut translation = tr.translation;

        let mut hex = grid
            .layout
            .world_pos_to_hex(hexx::Vec2::new(translation.x, translation.z));

        // hard check to make sure the projectile is inside the grid bounds.
        let (hex_radius, _) = grid.layout.hex_size.into();
        const SKIN_WIDTH: f32 = 0.1;
        let radius = hex_radius + SKIN_WIDTH;
        let (clamped, was_clamped, _) =
            clamp_inside_world_bounds(translation, radius, &grid.bounds);
        if was_clamped {
            hex = grid
                .layout
                .world_pos_to_hex(hexx::Vec2::new(clamped.x, clamped.z));
        }

        // Dumb iterative check to make sure chosen hex is not occupied.
        const MAX_ITER: usize = 10;
        let mut iter = 0;
        while let Some(_) = grid.get(hex) {
            let step_size = Vec3::Z * radius;
            translation += step_size;
            (translation, _, _) = clamp_inside_world_bounds(translation, radius, &grid.bounds);

            hex = grid
                .layout
                .world_pos_to_hex(hexx::Vec2::new(translation.x, translation.z));

            iter += 1;
            if iter >= MAX_ITER {
                break;
            }
        }

        let (x, z) = grid.layout.hex_to_world_pos(hex).into();
        let ball = commands
            .spawn((
                BallBundle::new(
                    Vec3::new(x, translation.y, z),
                    grid.layout.hex_size.x,
                    *species,
                    &mut meshes,
                    &mut materials,
                ),
                HexComponent { hex },
            ))
            .id();

        grid.set(hex, Some(ball));

        let (cluster, _) = find_cluster(&grid, hex, |&e| {
            e == ball
                || match balls.get(e) {
                    Ok(other) => *other == *species,
                    Err(_) => false,
                }
        });

        let mut score_add = 0;

        // remove matching clusters
        const MIN_CLUSTER_SIZE: usize = 3;
        if cluster.len() >= MIN_CLUSTER_SIZE {
            cluster.iter().for_each(|&hex| {
                commands.entity(*grid.get(hex).unwrap()).despawn();
                grid.set(hex, None);
                score_add += 1;
            });
        }

        // remove floating clusters
        let floating_clusters = find_floating_clusters(&grid);
        floating_clusters
            .iter()
            .flat_map(|e| e.iter())
            .for_each(|&hex| {
                commands.entity(*grid.get(hex).unwrap()).despawn();
                grid.set(hex, None);
                score_add += 1;
            });

        if score_add > 0 {
            commands.spawn((AudioBundle {
                source: audio_assets.score.clone_weak(),
                settings: PlaybackSettings::DESPAWN,
                ..default()
            },));
        }

        if turn_counter.0 % MOVE_DOWN_TURN == 0 {
            round_turn_counter.0 = 0;
            move_down_and_spawn.send(MoveDownAndSpawn);
        }

        // remove floating clusters
        let floating_clusters = find_floating_clusters(&grid);
        floating_clusters
            .iter()
            .flat_map(|e| e.iter())
            .for_each(|&hex| {
                commands.entity(*grid.get(hex).unwrap()).despawn();
                grid.set(hex, None);
                score_add += 1;
            });

        score.0 += score_add;

        begin_turn.send(BeginTurn);
    }
}
