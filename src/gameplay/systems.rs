use bevy::prelude::{
    Commands, DespawnRecursiveExt, EventReader, EventWriter, Input, KeyCode, NextState, Query, Res,
    ResMut, Vec2, Vec3, With,
};
use bevy_pkv::PkvStore;
use hexx::{Direction, Hex};

use crate::{
    components::AppState,
    game_audio::{constants::SFX_SOUND_VOLUME_KEY, utils::play_score_audio},
    gameplay::{
        ball::{grid_ball_bundle::GridBallBundle, utils::clamp_inside_world_bounds},
        constants::MIN_CLUSTER_SIZE,
        grid::{
            components::HexComponent,
            utils::{find_cluster, find_floating_clusters},
        },
    },
    loading::audio_assets::AudioAssets,
    resources::LevelCounter,
    utils::increment_level,
};

use super::{
    ball::{
        components::{GridBall, Species},
        events::SnapProjectile,
    },
    constants::PLAYER_SPAWN_Z,
    events::BeginTurn,
    grid::{events::UpdatePositions, resources::Grid},
    materials::resources::GameplayMaterials,
    meshes::resources::GameplayMeshes,
    panels::resources::{MoveCounter, ScoreCounter, TurnCounter},
};

pub fn setup_gameplay(mut begin_turn: EventWriter<BeginTurn>) {
    begin_turn.send(BeginTurn);
}

pub fn on_begin_turn(
    mut begin_turn: EventReader<BeginTurn>,
    mut update_positions: EventWriter<UpdatePositions>,
) {
    if begin_turn.is_empty() {
        return;
    }
    begin_turn.clear();

    update_positions.send(UpdatePositions);
}

pub fn check_game_over(grid: Res<Grid>, mut app_state_next_state: ResMut<NextState<AppState>>) {
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

    for (&hex, _) in grid.storage.iter() {
        let world_pos = grid.layout.hex_to_world_pos(hex);
        if world_pos.y >= z - 0.1 {
            app_state_next_state.set(AppState::GameOver);
            break;
        }
    }
}

pub fn check_game_win(
    grid: Res<Grid>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut level_counter: ResMut<LevelCounter>,
) {
    if grid.storage.len() == 0 {
        increment_level(&mut level_counter);
        app_state_next_state.set(AppState::GameWin);
    }
}

pub fn on_snap_projectile(
    mut snap_projectile_events: EventReader<SnapProjectile>,
    mut commands: Commands,
    gameplay_meshes: Res<GameplayMeshes>,
    gameplay_materials: Res<GameplayMaterials>,
    mut grid: ResMut<Grid>,
    mut begin_turn: EventWriter<BeginTurn>,
    mut score_counter: ResMut<ScoreCounter>,
    mut turn_counter: ResMut<TurnCounter>,
    mut move_counter: ResMut<MoveCounter>,
    balls_query: Query<&Species, With<GridBall>>,
    audio_assets: Res<AudioAssets>,
    pkv: Res<PkvStore>,
) {
    if let Some(snap_projectile) = snap_projectile_events.iter().next() {
        let mut hex = grid.layout.world_pos_to_hex(snap_projectile.pos);

        if !snap_projectile.out_of_bounds {
            // hard check to make sure the projectile is inside the grid bounds.
            let (hex_radius, _) = grid.layout.hex_size.into();
            const SKIN_WIDTH: f32 = 0.1;
            let radius = hex_radius + SKIN_WIDTH;
            let (clamped, was_clamped, _) =
                clamp_inside_world_bounds(snap_projectile.pos, radius, &grid.bounds);
            if was_clamped {
                hex = grid.layout.world_pos_to_hex(clamped);
            }

            // Dumb iterative check to make sure chosen hex is not occupied.
            const MAX_ITER: usize = 10;
            let mut iter = 0;
            let mut fine_pos = snap_projectile.pos;
            while let Some(_) = grid.get(hex) {
                let step_size = Vec2::Y * radius;
                fine_pos += step_size;
                (fine_pos, _, _) = clamp_inside_world_bounds(fine_pos, radius, &grid.bounds);

                hex = grid.layout.world_pos_to_hex(fine_pos);

                iter += 1;
                if iter >= MAX_ITER {
                    break;
                }
            }
        }

        let (x, z) = grid.layout.hex_to_world_pos(hex).into();
        let ball = commands
            .spawn((
                GridBallBundle::new(
                    Vec3::new(x, 0.0, z),
                    grid.layout.hex_size.x,
                    snap_projectile.species,
                    &gameplay_meshes,
                    &gameplay_materials,
                ),
                HexComponent { hex },
            ))
            .id();

        grid.set(hex, ball); // add snapped projectile ball as grid ball

        let mut score_add = 0;

        let neighbors = grid.neighbors(hex);
        if neighbors.len() == 0 {
            // projectile ball snapped with no neghbours
            // do not calc floating clusters
        } else {
            let (cluster, _) = find_cluster(&grid, hex, |&e| {
                e == ball
                    || match balls_query.get(e) {
                        Ok(other) => *other == snap_projectile.species,
                        Err(_) => false,
                    }
            });

            // remove matching clusters
            if cluster.len() >= MIN_CLUSTER_SIZE {
                cluster.iter().for_each(|&hex| {
                    commands.entity(*grid.get(hex).unwrap()).despawn_recursive();
                    grid.remove(&hex);
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
                    grid.remove(&hex);
                    score_add += 1;
                });

            if score_add > 0 {
                if let Ok(sfx_sound_volume) = pkv.get::<String>(SFX_SOUND_VOLUME_KEY) {
                    if let Ok(sfx_sound_volume) = sfx_sound_volume.parse::<f32>() {
                        if sfx_sound_volume > 0.0 {
                            play_score_audio(&mut commands, &audio_assets, sfx_sound_volume);
                        }
                    }
                }
            }

            score_counter.0 += score_add;
        }

        turn_counter.0 += 1;
        if score_add == 0 {
            move_counter.0 += 1;
        }

        begin_turn.send(BeginTurn);

        grid.update_bounds();
    }
    snap_projectile_events.clear();
}

pub fn keydown_detect(
    mut app_state_next_state: ResMut<NextState<AppState>>,
    keyboard_input_key_code: Res<Input<KeyCode>>,
    mut level_counter: ResMut<LevelCounter>,
) {
    if keyboard_input_key_code.any_just_released([KeyCode::Escape]) {
        app_state_next_state.set(AppState::GameOver);
    }
    if keyboard_input_key_code.any_just_released([KeyCode::Space]) {
        increment_level(&mut level_counter);
        app_state_next_state.set(AppState::GameWin);
    }
}
