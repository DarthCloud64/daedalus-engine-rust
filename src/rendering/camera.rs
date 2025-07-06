use bytemuck::{Pod, Zeroable};
use glam::Mat4;

use crate::ecs::component::{camera::CameraComponent, transform::TransformComponent};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct CameraUniform {
    pub view_projection_matrix: [[f32; 4]; 4], // 4x4 matrix for the view-projection transformation
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_projection_matrix: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_projection_matrix(
        &mut self,
        camera_component: &CameraComponent,
        transform_component: &TransformComponent,
    ) {
        self.view_projection_matrix = camera_component
            .calculate_view_projection_matrix(transform_component)
            .to_cols_array_2d();
    }
}
