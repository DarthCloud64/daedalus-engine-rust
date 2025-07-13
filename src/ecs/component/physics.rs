use glam::Vec3;

#[derive(Debug, Default, Clone, Copy)]
pub struct PhysicsComponent {
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub speed: f32,
}
