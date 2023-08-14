use bevy::{
    prelude::{default, BuildChildren, Bundle, Commands, Entity, Res, Transform, Vec2, Vec3},
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::{
    prelude::{Collider, CollisionGroups, Damping, Group, RigidBody, Velocity},
    render::ColliderDebugColor,
};
use hexx::Hex;

use crate::gameplay::{
    constants::BALL_RADIUS,
    grid::{
        resources::Grid,
        utils::{build_ball_text, build_joints},
    },
    materials::resources::GameplayMaterials,
    meshes::resources::GameplayMeshes,
};

use super::components::{GridBall, LastActiveGridBall, Species};

pub struct GridBallBundle;

impl GridBallBundle {
    fn new(
        pos: Vec2,
        species: Species,
        gameplay_meshes: &Res<GameplayMeshes>,
        gameplay_materials: &Res<GameplayMaterials>,
        hex: Hex,
        rigid_body: RigidBody,
    ) -> impl Bundle {
        (
            MaterialMesh2dBundle {
                mesh: gameplay_meshes.grid_ball.clone().into(),
                material: gameplay_materials.from_species(species),
                transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
                ..default()
            },
            GridBall { hex },
            species,
            rigid_body,
            Collider::ball(BALL_RADIUS),
            ColliderDebugColor(species.into()),
            CollisionGroups::new(Group::GROUP_2, Group::GROUP_1 | Group::GROUP_3),
            Velocity::default(),
            Damping {
                linear_damping: 0.5,
                angular_damping: 0.5,
            },
        )
    }

    pub fn spawn(
        commands: &mut Commands,
        grid: &Grid,
        gameplay_meshes: &Res<GameplayMeshes>,
        gameplay_materials: &Res<GameplayMaterials>,
        hex: Hex,
        is_last_active: bool,
        some_species: Option<Species>,
    ) -> Entity {
        let hex_pos = grid.layout.hex_to_world_pos(hex);

        let mut entity_commands = commands.spawn(Self::new(
            hex_pos,
            match some_species {
                Some(species) => species,
                None => Species::random_species(),
            },
            &gameplay_meshes,
            &gameplay_materials,
            hex,
            match is_last_active {
                true => RigidBody::KinematicPositionBased,
                false => RigidBody::Dynamic,
            },
        ));

        if is_last_active {
            println!("insert LastActiveGridBall {:?}", hex);
            entity_commands.insert(LastActiveGridBall {});
        }

        entity_commands
            .with_children(|parent| {
                if !is_last_active {
                    for joint in build_joints(hex, &grid) {
                        parent.spawn(joint);
                    }
                }
                build_ball_text(parent, hex);
            })
            .id()
    }
}
