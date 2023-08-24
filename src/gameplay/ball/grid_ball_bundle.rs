use bevy::{
    prelude::{default, BuildChildren, Bundle, Commands, Entity, Res, Transform, Vec2, Vec3},
    sprite::MaterialMesh2dBundle,
};
use bevy_xpbd_2d::prelude::{
    AngularDamping, CoefficientCombine, Collider, CollisionLayers, ExternalForce, LockedAxes,
    Position, Restitution, RigidBody,
};

use crate::gameplay::{
    constants::BALL_RADIUS, grid::utils::build_ball_text, materials::resources::GameplayMaterials,
    meshes::resources::GameplayMeshes, physics::layers::Layer,
};

use super::components::{
    GridBall, GridBallScaleAnimate, MagneticGridBall, ProjectileBall, Species,
};

pub struct GridBallBundle;

impl GridBallBundle {
    fn new(
        transform: Transform,
        pos: Vec2,
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
            MagneticGridBall {},
            species,
            RigidBody::Dynamic,
            Collider::ball(BALL_RADIUS),
            Position(pos),
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombine::Multiply,
            },
            AngularDamping(0.5),
            CollisionLayers::new([Layer::Grid], [Layer::Walls, Layer::Grid]),
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
        let mut transform = Transform::default();
        if is_appear_animation {
            transform = transform.with_scale(Vec3::ZERO);
        }
        let species = match some_species {
            Some(species) => species,
            None => Species::random_species(),
        };

        let mut entity_commands = commands.spawn(Self::new(
            transform,
            position,
            species,
            &gameplay_meshes,
            &gameplay_materials,
        ));

        if is_last_active {
            entity_commands.insert(LockedAxes::TRANSLATION_LOCKED);
        }
        if is_appear_animation {
            entity_commands.insert(GridBallScaleAnimate::from_scale(Vec2::ONE));
        }
        if is_projectile {
            entity_commands
                .insert(ProjectileBall::default())
                .insert(ExternalForce::new(Vec2::ONE * 100.0));
        }
        if debug_text {
            entity_commands.with_children(|parent| {
                build_ball_text(parent, None);
            });
        }

        (entity_commands.id(), species)
    }
}
