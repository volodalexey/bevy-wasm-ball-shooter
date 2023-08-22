use bevy::{
    prelude::{default, BuildChildren, Bundle, Commands, Entity, Res, Transform, Vec2, Vec3},
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::{
    prelude::{
        ActiveEvents, CoefficientCombineRule, Collider, CollisionGroups, Damping, ExternalForce,
        Friction, Group, LockedAxes, Restitution, RigidBody, Velocity,
    },
    render::ColliderDebugColor,
};

use crate::gameplay::{
    constants::BALL_RADIUS, grid::utils::build_ball_text, materials::resources::GameplayMaterials,
    meshes::resources::GameplayMeshes,
};

use super::components::{
    GridBall, GridBallScaleAnimate, MagneticGridBall, ProjectileBall, Species,
};

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
            RigidBody::Dynamic,
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
                angular_damping: 0.1,
            },
            Friction {
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
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
        is_projectile: bool,
        some_species: Option<Species>,
        is_appear_animation: bool,
        debug_text: bool,
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
        entity_commands.insert(MagneticGridBall {});

        if is_last_active {
            entity_commands.insert(LockedAxes::all());
        }
        if is_appear_animation {
            entity_commands.insert(GridBallScaleAnimate::from_scale(Vec2::ONE));
        }
        if is_projectile {
            entity_commands.insert(ProjectileBall::default());
        }
        if debug_text {
            entity_commands.with_children(|parent| {
                build_ball_text(parent, None);
            });
        }

        (entity_commands.id(), species)
    }
}
