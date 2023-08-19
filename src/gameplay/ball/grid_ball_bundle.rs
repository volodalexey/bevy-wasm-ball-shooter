use bevy::{
    prelude::{default, Bundle, Commands, Entity, Res, Transform, Vec2, Vec3},
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::{
    prelude::{
        ActiveEvents, Collider, CollisionGroups, Damping, ExternalForce, Group, RigidBody, Velocity,
    },
    render::ColliderDebugColor,
};

use crate::gameplay::{
    constants::BALL_RADIUS, materials::resources::GameplayMaterials,
    meshes::resources::GameplayMeshes,
};

use super::components::{GridBall, GridBallScaleAnimate, LastActiveGridBall, Species};

pub struct GridBallBundle;

impl GridBallBundle {
    fn new(
        transform: Transform,
        species: Species,
        gameplay_meshes: &Res<GameplayMeshes>,
        gameplay_materials: &Res<GameplayMaterials>,
    ) -> impl Bundle {
        (
            MaterialMesh2dBundle {
                mesh: gameplay_meshes.grid_ball.clone().into(),
                material: gameplay_materials.from_species(species),
                transform,
                ..default()
            },
            GridBall::default(),
            species,
            Collider::ball(BALL_RADIUS),
            ColliderDebugColor(species.into()),
            CollisionGroups::new(
                Group::GROUP_2,
                Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_3,
            ),
            ActiveEvents::COLLISION_EVENTS,
            Velocity::default(),
            Damping {
                linear_damping: 0.5,
                angular_damping: 0.5,
            },
            ExternalForce::default(),
        )
    }

    pub fn spawn(
        commands: &mut Commands,
        gameplay_meshes: &Res<GameplayMeshes>,
        gameplay_materials: &Res<GameplayMaterials>,
        position: Vec2,
        is_last_active: bool,
        some_species: Option<Species>,
        is_appear_animation: bool,
    ) -> (Entity, Species) {
        let mut transform = Transform::from_translation(position.extend(0.0));
        if is_appear_animation {
            transform = transform.with_scale(Vec3::ZERO);
        }
        let species = match some_species {
            Some(species) => species,
            None => Species::random_species(),
        };

        let mut entity_commands = commands.spawn(Self::new(
            transform,
            species,
            &gameplay_meshes,
            &gameplay_materials,
        ));

        if is_last_active {
            entity_commands
                .insert(LastActiveGridBall {})
                .insert(RigidBody::KinematicPositionBased);
        } else {
            entity_commands.insert(RigidBody::Dynamic);
        }
        if is_appear_animation {
            entity_commands.insert(GridBallScaleAnimate::from_scale(Vec2::ONE));
        }

        (entity_commands.id(), species)
    }
}
