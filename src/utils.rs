use nalgebra_glm::{Vec3, Vec4, Mat4};



pub fn frustum_culling(
    position: &Vec3,
    scale: f32,
    view_matrix: &Mat4,
    projection_matrix: &Mat4,
) -> bool {
    let world_pos = Vec4::new(position.x, position.y, position.z, 1.0);
    let clip_space_pos = projection_matrix * view_matrix * world_pos;
    let margin = scale * 1.5;

    // Dividir por w para obtener coordenadas NDC (Normalized Device Coordinates)
    let w = clip_space_pos.w;
    if w.abs() < f32::EPSILON {
        return false; // Prevenir divisiones por cero
    }
    let ndc_x = clip_space_pos.x / w;
    let ndc_y = clip_space_pos.y / w;
    let ndc_z = clip_space_pos.z / w;

    // Verificar si está dentro del frustum considerando el margen
    ndc_x.abs() <= 1.0 + margin
        && ndc_y.abs() <= 1.0 + margin
        && ndc_z >= -1.0 - margin
        && ndc_z <= 1.0 + margin
}
