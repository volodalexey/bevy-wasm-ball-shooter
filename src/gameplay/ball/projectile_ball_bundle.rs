use bevy::prelude::{Bundle, PbrBundle, Res, Transform, Vec3};
use bevy_rapier3d::prelude::{ActiveEvents, Collider, LockedAxes, RigidBody, Velocity};

use crate::gameplay::{materials::resources::GameplayMaterials, meshes::resources::GameplayMeshes};

use super::{
    components::{ProjectileBall, Species},
    constants::BALL_RADIUS_COEFF,
};

#[derive(Bundle)]
pub struct ProjectileBallBundle {
    pub pbr: PbrBundle,
    pub ball: ProjectileBall,
    pub species: Species,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub locked_axes: LockedAxes,
    pub velocity: Velocity,
    pub collision_events: ActiveEvents,
}

impl ProjectileBallBundle {
    pub fn new(
        pos: Vec3,
        radius: f32,
        species: Species,
        gameplay_meshes: &Res<GameplayMeshes>,
        gameplay_materials: &Res<GameplayMaterials>,
    ) -> Self {
        Self {
            pbr: PbrBundle {
                mesh: gameplay_meshes.grid_ball.clone(),
                material: gameplay_materials.from_species(species),
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
            ball: ProjectileBall { is_flying: false },
            species,
            collider: Collider::ball(radius * BALL_RADIUS_COEFF),
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::TRANSLATION_LOCKED_Z,
            velocity: Velocity::default(),
            collision_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}
