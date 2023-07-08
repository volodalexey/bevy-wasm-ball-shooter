use bevy::prelude::{
    shape::Icosphere, Assets, Bundle, Mesh, PbrBundle, ResMut, StandardMaterial, Transform, Vec3,
};
use bevy_rapier3d::prelude::{ActiveEvents, Ccd, Collider, RigidBody, Velocity};

use crate::gameplay::ball::{species_to_color, Species, BALL_RADIUS_COEFF};

use super::{
    components::{Flying, Projectile},
    constants::PROJ_COLLIDER_COEFF,
};

#[derive(Bundle)]
pub struct ProjectileBundle {
    #[bundle]
    pub pbr: PbrBundle,
    pub rigid_body: RigidBody,
    pub ccd: Ccd,
    pub collider: Collider,
    pub velocity: Velocity,
    pub collision_events: ActiveEvents,
    pub projectile: Projectile,
    pub is_flying: Flying,
    pub species: Species,
}

impl ProjectileBundle {
    pub fn new(
        pos: Vec3,
        radius: f32,
        species: Species,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        Self {
            pbr: PbrBundle {
                mesh: meshes.add(
                    Mesh::try_from(Icosphere {
                        radius: radius * BALL_RADIUS_COEFF,
                        subdivisions: 1,
                    })
                    .expect("Unable to generate IcoSphere"),
                ),
                material: materials.add(species_to_color(species).into()),
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
            collider: Collider::ball(radius * BALL_RADIUS_COEFF * PROJ_COLLIDER_COEFF),
            is_flying: Flying(false),
            species: species,
            ..Default::default()
        }
    }
}

impl Default for ProjectileBundle {
    fn default() -> Self {
        ProjectileBundle {
            pbr: Default::default(),
            rigid_body: RigidBody::KinematicVelocityBased,
            collider: Collider::ball(1.),
            collision_events: ActiveEvents::all(),
            projectile: Projectile,
            is_flying: Flying(false),
            velocity: Velocity::linear(Vec3::new(0., 0., 0.)),
            ccd: Ccd::enabled(),
            species: Species::Red,
        }
    }
}
