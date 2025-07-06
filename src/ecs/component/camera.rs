use glam::{Mat4, Vec3, Vec4};

use crate::ecs::component::transform::TransformComponent;

const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols(
    Vec4::new(1.0, 0.0, 0.0, 0.0),
    Vec4::new(0.0, 1.0, 0.0, 0.0),
    Vec4::new(0.0, 0.0, 0.5, 0.0),
    Vec4::new(0.0, 0.0, 0.5, 1.0),
);

#[derive(Debug, Clone, Copy)]
pub struct CameraComponent {
    pub look_at: Vec3,
    pub up_orientation: Vec3,
    pub aspect_ratio: f32,
    pub field_of_view: f32,
    pub z_near_field: f32, // Closest distance to the camera that things are rendered
    pub z_far_field: f32,  // Farthest distance to the camera that things are rendered
}

impl CameraComponent {
    pub fn calculate_view_projection_matrix(
        &self,
        transform_component: &TransformComponent,
    ) -> Mat4 {
        // Moves the world to be at the position the camera is looking at
        let view_matrix = Mat4::look_at_rh(
            transform_component.position,
            self.look_at,
            self.up_orientation,
        );

        // Warps the scene to provide depth
        let projection_matrix = Mat4::perspective_rh(
            self.field_of_view.to_radians(),
            self.aspect_ratio,
            self.z_near_field,
            self.z_far_field,
        );

        OPENGL_TO_WGPU_MATRIX * projection_matrix * view_matrix
    }
}
