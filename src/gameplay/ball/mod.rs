use bevy::prelude::{
    apply_deferred, in_state, App, FixedUpdate, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update,
};

use crate::components::AppState;

use self::{
    resources::ProjectileHelper,
    systems::{
        aim_systems::{cleanup_aim_lines, cleanup_aim_target, draw_aim, setup_aim_target},
        out_ball_systems::{animate_out_ball, check_out_ball_for_delete},
        projectile_systems::{
            check_projectile_species, cleanup_next_projectile_ball, cleanup_projectile_ball,
            projectile_reload, shoot_projectile,
        },
        scale_systems::animate_grid_ball_scale,
    },
};

pub mod aim_bundle;
pub mod components;
pub mod grid_ball_bundle;
pub mod out_ball_bundle;
pub mod projectile_ball_bundle;
mod resources;
mod systems;
mod utils;
pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ProjectileHelper>()
            .add_systems(OnEnter(AppState::Gameplay), setup_aim_target)
            .add_systems(
                Update,
                (
                    projectile_reload.after(apply_deferred),
                    shoot_projectile,
                    animate_out_ball,
                    check_out_ball_for_delete,
                    draw_aim,
                    animate_grid_ball_scale,
                )
                    .run_if(in_state(AppState::Gameplay)),
            )
            .add_systems(
                FixedUpdate,
                check_projectile_species.run_if(in_state(AppState::Gameplay)),
            )
            .add_systems(
                OnExit(AppState::Gameplay),
                (
                    cleanup_projectile_ball,
                    cleanup_aim_target,
                    cleanup_aim_lines,
                    cleanup_next_projectile_ball,
                ),
            );
    }
}
