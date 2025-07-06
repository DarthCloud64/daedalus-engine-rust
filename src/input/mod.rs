use log::{info, trace};
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode};

use crate::ecs::entity::scene::Scene;

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
            match (code, is_pressed) {
                (KeyCode::Escape, true) => {
                    info!("Escape key pressed, exiting...");
                    event_loop.exit();
                }

                (KeyCode::ArrowUp, true) => {
                    info!("Moving forward for entity: {:?}", current_entity);
                    input_component.moving_forward = true;
                }
                (KeyCode::ArrowUp, false) => {
                    info!("Stopped moving forward for entity: {:?}", current_entity);
                    input_component.moving_forward = false;
                }

                (KeyCode::ArrowDown, true) => {
                    info!("Moving backward for entity: {:?}", current_entity);
                    input_component.moving_backward = true;
                }
                (KeyCode::ArrowDown, false) => {
                    info!("Stopped moving backward for entity: {:?}", current_entity);
                    input_component.moving_backward = false;
                }

                (KeyCode::ArrowLeft, true) => {
                    info!("Moving left for entity: {:?}", current_entity);
                    input_component.moving_left = true;
                }
                (KeyCode::ArrowLeft, false) => {
                    info!("Stopped moving left for entity: {:?}", current_entity);
                    input_component.moving_left = false;
                }

                (KeyCode::ArrowRight, true) => {
                    info!("Moving right for entity: {:?}", current_entity);
                    input_component.moving_right = true;
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
