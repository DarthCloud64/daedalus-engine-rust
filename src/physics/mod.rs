use glam::Vec3;

use crate::ecs::entity::scene::Scene;

pub struct PhysicsService {}

impl PhysicsService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_physics(&self, scene: &mut Scene, delta_time: f32) {
        for (current_entity, physics_component) in scene.physics_components.iter_mut() {
            // Update acceleration based on input
            if let Some(input_component) = scene.input_components.get(current_entity) {
                let mut something_pressed = false;

                // Reset acceleration
                physics_component.acceleration = Vec3::ZERO;
                if input_component.up_pressed {
                    physics_component.acceleration.y += physics_component.speed;
                    something_pressed = true;
                }
                if input_component.down_pressed {
                    physics_component.acceleration.y -= physics_component.speed;
                    something_pressed = true;
                }
                if input_component.left_pressed {
                    physics_component.acceleration.x -= physics_component.speed;
                    something_pressed = true;
                }
                if input_component.right_pressed {
                    physics_component.acceleration.x += physics_component.speed;
                    something_pressed = true;
                }

                if something_pressed {
                    // Update velocity using acceleration
                    physics_component.velocity += physics_component.acceleration * delta_time;
                } else {
                    // If no input, stop movement
                    physics_component.velocity = Vec3::ZERO;
                }
            }

            // Update position if transform exists
            if let Some(transform_component) = scene.transform_components.get_mut(current_entity) {
                transform_component.position.x += physics_component.velocity.x * delta_time;
                transform_component.position.y += physics_component.velocity.y * delta_time;
            }
        }
    }
}
