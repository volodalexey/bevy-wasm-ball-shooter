use std::ops::Add;

use bevy::{
    prelude::{
        warn, Assets, Commands, DespawnRecursiveExt, Entity, Mesh, Query, Res, ResMut, Transform,
        Vec2, Visibility, With, Without,
    },
    utils::HashSet,
};
use bevy_xpbd_2d::prelude::{Collider, ShapeHitData, SpatialQuery, SpatialQueryFilter};

use crate::gameplay::{
    ball::{
        aim_bundle::AimBundle,
        components::{AimLine, AimTarget, GridBall, ProjectileBall},
        utils::cleanup_aim_line_utils,
    },
    constants::{
        BALL_RADIUS, CAST_RAY_BOUNCE_Y_ADD, CAST_RAY_MAX_TOI, CAST_RAY_TRIES, CAST_RAY_VELOCITY,
        CAST_RAY_VELOCITY_TOLERANCE,
    },
    grid::resources::Grid,
    lines::components::LineType,
    materials::resources::GameplayMaterials,
    physics::layers::Layer,
    walls::components::WallType,
};

pub fn draw_aim(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    gameplay_materials: Res<GameplayMaterials>,
    wall_query: Query<(Entity, &WallType), With<WallType>>,
    balls_query: Query<Entity, With<GridBall>>,
    spatial_query: SpatialQuery,
    aim_line_query: Query<Entity, With<AimLine>>,
    mut aim_target_query: Query<
        (&mut AimTarget, &mut Transform, &mut Visibility),
        (With<AimTarget>, Without<LineType>),
    >,
    projectile_ball_query: Query<Entity, With<ProjectileBall>>,
) {
    if let Ok((mut aim_target, mut target_transform, mut target_visibility)) =
        aim_target_query.get_single_mut()
    {
        if aim_target.pointer_released {
            *target_visibility = Visibility::Hidden;
            cleanup_aim_line_utils(&mut commands, &aim_line_query);
            aim_target.draw_vel = Vec2::ZERO;
        }

        if aim_target.pointer_pressed && aim_target.draw_vel != aim_target.aim_vel {
            aim_target.draw_vel = aim_target.aim_vel;
            // redraw only if pointer position (draw velocity in this case) changed

            cleanup_aim_line_utils(&mut commands, &aim_line_query);
            let mut exclude_entities: HashSet<Entity> = HashSet::default();
            for projectile_entity in projectile_ball_query.iter() {
                exclude_entities.insert(projectile_entity);
            }
            let spatial_query_filter = SpatialQueryFilter::default()
                .with_masks([Layer::Walls, Layer::Grid])
                .without_entities(exclude_entities);

            let shape = Collider::ball(BALL_RADIUS);
            let mut ray_start = aim_target.aim_pos;
            let mut ray_vel = aim_target.aim_vel.normalize() * CAST_RAY_VELOCITY;
            let mut count = 0;

            loop {
                count = count + 1;
                if count > CAST_RAY_TRIES {
                    let message = format!("Break aim iteration draw, reached max iteration count aim.aim_pos({:?}) aim_vel({:?})", aim_target.aim_pos, aim_target.aim_vel);
                    warn!(message);
                    break;
                }
                if let Some(ShapeHitData {
                    entity,
                    time_of_impact: _,
                    point1,
                    point2,
                    normal1: _,
                    normal2: _,
                }) = spatial_query.cast_shape(
                    &shape,
                    ray_start,
                    0.0,
                    ray_vel,
                    CAST_RAY_MAX_TOI,
                    true,
                    spatial_query_filter.clone(),
                ) {
                    if let Ok((_, wall_type)) = wall_query.get(entity) {
                        let mut center = point1 - point2;
                        if wall_type.is_side() {
                            center = center.add(Vec2::new(
                                0.0,
                                CAST_RAY_BOUNCE_Y_ADD * ray_vel.normalize().y,
                            ));
                        } else {
                            target_transform.translation =
                                center.extend(target_transform.translation.z);
                            *target_visibility = Visibility::Visible;
                        }
                        commands.spawn(AimBundle::new_line(
                            ray_start,
                            center,
                            &mut meshes,
                            &gameplay_materials,
                        ));
                        if wall_type.is_top() {
                            break;
                        }
                        ray_start = center;
                        let mut reverse_x_vel = -ray_vel.x;
                        match reverse_x_vel.signum() > 0.0 {
                            true => reverse_x_vel += CAST_RAY_VELOCITY_TOLERANCE,
                            false => reverse_x_vel -= CAST_RAY_VELOCITY_TOLERANCE,
                        }
                        ray_vel = Vec2::new(reverse_x_vel, ray_vel.y);
                    } else if let Ok(_) = balls_query.get(entity) {
                        let center = point1 - point2;
                        target_transform.translation =
                            center.extend(target_transform.translation.z);
                        commands.spawn(AimBundle::new_line(
                            ray_start,
                            center,
                            &mut meshes,
                            &gameplay_materials,
                        ));
                        *target_visibility = Visibility::Visible;
                        break;
                    }
                }
            }
        }
    }
}

pub fn setup_aim_target(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    gameplay_materials: Res<GameplayMaterials>,
    grid: Res<Grid>,
) {
    commands.spawn(AimBundle::new_target(
        Vec2::new(0.0, 0.0),
        &mut meshes,
        &gameplay_materials,
        &grid,
    ));
}

pub fn cleanup_aim_target(
    mut commands: Commands,
    aim_target_query: Query<Entity, With<AimTarget>>,
) {
    for projectile_entity in aim_target_query.iter() {
        commands.entity(projectile_entity).despawn_recursive();
    }
}

pub fn cleanup_aim_lines(mut commands: Commands, aim_line_query: Query<Entity, With<AimLine>>) {
    cleanup_aim_line_utils(&mut commands, &aim_line_query);
}
