use log::{info, trace};
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode};

use crate::ecs::{component::transform::TransformComponent, entity::scene::Scene};

pub struct InputService {}

impl InputService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_input(
        &self,
        event_loop: &ActiveEventLoop,
        code: KeyCode,
        is_pressed: bool,
        scene: &mut Scene,
    ) {
        for (current_entity, input_component) in scene.input_components.iter_mut() {
            let transform_component = scene.transform_components.get_mut(current_entity).unwrap();

            match (code, is_pressed) {
                (KeyCode::Escape, true) => {
                    info!("Escape key pressed, exiting...");
                    event_loop.exit();
                }

                (KeyCode::ArrowUp, true) => {
                    info!("Moving up for entity: {:?}", current_entity);
                    input_component.moving_up = true;
                    transform_component.position.y += 1.0; // Example movement logic
                }
                (KeyCode::ArrowUp, false) => {
                    info!("Stopped moving up for entity: {:?}", current_entity);
                    input_component.moving_up = false;
                }

                (KeyCode::ArrowDown, true) => {
                    info!("Moving down for entity: {:?}", current_entity);
                    input_component.moving_down = true;
                    transform_component.position.y -= 1.0; // Example movement logic
                }
                (KeyCode::ArrowDown, false) => {
                    info!("Stopped moving down for entity: {:?}", current_entity);
                    input_component.moving_down = false;
                }

                (KeyCode::ArrowLeft, true) => {
                    info!("Moving left for entity: {:?}", current_entity);
                    input_component.moving_left = true;
                    transform_component.position.x -= 1.0; // Example movement logic
                }
                (KeyCode::ArrowLeft, false) => {
                    info!("Stopped moving left for entity: {:?}", current_entity);
                    input_component.moving_left = false;
                }

                (KeyCode::ArrowRight, true) => {
                    info!("Moving right for entity: {:?}", current_entity);
                    input_component.moving_right = true;
                    transform_component.position.x += 1.0; // Example movement logic
                }
                (KeyCode::ArrowRight, false) => {
                    info!("Stopped moving right for entity: {:?}", current_entity);
                    input_component.moving_right = false;
                }

                _ => {
                    info!("Other key event: {:?}", code);
                }
            }
        }
    }
}
