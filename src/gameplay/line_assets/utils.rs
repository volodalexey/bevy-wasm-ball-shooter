use bevy::{
    ecs::query::QuerySingleError,
    prelude::{Assets, Color, Handle, Mesh, Mut, ResMut, StandardMaterial, Vec3, Visibility},
    render::mesh::VertexAttributeValues,
};

pub fn draw_line(
    line_result: Result<
        (
            Mut<'_, Handle<Mesh>>,
            Mut<'_, Handle<StandardMaterial>>,
            Mut<'_, Visibility>,
        ),
        QuerySingleError,
    >,
    meshes: &mut ResMut<Assets<Mesh>>,
    materilas: &mut ResMut<Assets<StandardMaterial>>,
    start: Vec3,
    end: Vec3,
    color: Color,
) {
    if let Ok((line_handle_mesh, line_handle_material, mut visibility)) = line_result {
        if let Some(line_mesh) = meshes.get_mut(&line_handle_mesh) {
            if let Some(vertex_attribute_values) = line_mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
            {
                if let VertexAttributeValues::Float32x3(mesh_values) = vertex_attribute_values {
                    if let Some(first) = mesh_values.first_mut() {
                        first[0] = start.x;
                        first[1] = start.y;
                        first[2] = start.z;
                    }
                    if let Some(last) = mesh_values.last_mut() {
                        last[0] = end.x;
                        last[1] = end.y;
                        last[2] = end.z;
                    }
                }
            }
        }
        if let Some(line_material) = materilas.get_mut(&line_handle_material) {
            line_material.emissive = color;
        }
        *visibility = Visibility::Visible;
    }
}
