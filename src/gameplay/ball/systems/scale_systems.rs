use bevy::{
    prelude::{Commands, Entity, Query, Res, Transform, With},
    time::Time,
};

use crate::gameplay::{ball::components::GridBallScaleAnimate, constants::APPEAR_TOLERANCE};

pub fn animate_grid_ball_scale(
    mut commands: Commands,
    mut grid_balls_query: Query<
        (Entity, &mut Transform, &mut GridBallScaleAnimate),
        With<GridBallScaleAnimate>,
    >,
    time: Res<Time>,
) {
    for (ball_entity, mut grid_ball_transform, mut grid_ball_animate) in grid_balls_query.iter_mut()
    {
        grid_ball_animate.timer.tick(time.delta());
        grid_ball_transform.scale = grid_ball_transform
            .scale
            .truncate()
            .lerp(grid_ball_animate.scale, grid_ball_animate.timer.percent())
            .extend(grid_ball_transform.scale.z);
        if (grid_ball_transform.scale.truncate() - grid_ball_animate.scale).length()
            < APPEAR_TOLERANCE
        {
            grid_ball_transform.scale = grid_ball_animate.scale.extend(grid_ball_transform.scale.z);
            commands
                .entity(ball_entity)
                .remove::<GridBallScaleAnimate>();
        }
    }
}
