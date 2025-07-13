use std::collections::HashMap;

use crate::ecs::component::{
    camera::CameraComponent, input::InputComponent, physics::PhysicsComponent,
    transform::TransformComponent,
};

#[derive(Debug, Default, Clone)]
pub struct Scene {
    next_entity_id: u32,
    pub transform_components: HashMap<u32, TransformComponent>,
    pub camera_components: HashMap<u32, CameraComponent>,
    pub input_components: HashMap<u32, InputComponent>,
    pub physics_components: HashMap<u32, PhysicsComponent>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            next_entity_id: 0,
            transform_components: HashMap::new(),
            camera_components: HashMap::new(),
            input_components: HashMap::new(),
            physics_components: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> u32 {
        self.next_entity_id += 1;

        self.next_entity_id
    }
}
