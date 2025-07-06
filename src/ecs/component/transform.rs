use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct TransformComponent {
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Vec3, // Euler angles in radians
    pub translation: Vec3,
}
