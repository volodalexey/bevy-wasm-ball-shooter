mod resources;
pub mod utils;

use bevy::{
    prelude::{
        default, App, Assets, Color, Commands, Mesh, Plugin, ResMut, StandardMaterial, Vec3,
    },
    render::render_resource::PrimitiveTopology,
};

pub use self::resources::LineAssets;

pub struct LineAssetsPlugin;

impl Plugin for LineAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LineAssets>()
            .add_startup_system(setup_assets);
    }
}

pub fn setup_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut line_list_mesh = Mesh::new(PrimitiveTopology::LineList);
    line_list_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![Vec3::ZERO, Vec3::ZERO]);
    let handle_mesh = meshes.add(line_list_mesh);

    let standard_material = StandardMaterial {
        emissive: Color::WHITE,
        ..default()
    };
    let handle_material = materials.add(standard_material);

    commands.insert_resource(LineAssets {
        mesh: handle_mesh,
        material: handle_material,
    });
}
