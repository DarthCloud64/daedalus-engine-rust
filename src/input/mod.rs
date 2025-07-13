use log::info;
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
                    info!("Up pressed for entity: {:?}", current_entity);
                    input_component.up_pressed = true;
                }
                (KeyCode::ArrowUp, false) => {
                    info!("Stopped pressing up for entity: {:?}", current_entity);
                    input_component.up_pressed = false;
                }

                (KeyCode::ArrowDown, true) => {
                    info!("Down pressed for entity: {:?}", current_entity);
                    input_component.down_pressed = true;
                }
                (KeyCode::ArrowDown, false) => {
                    info!("Stopped pressing down for entity: {:?}", current_entity);
                    input_component.down_pressed = false;
                }

                (KeyCode::ArrowLeft, true) => {
                    info!("Left pressed for entity: {:?}", current_entity);
                    input_component.left_pressed = true;
                }
                (KeyCode::ArrowLeft, false) => {
                    info!("Stopped pressing left for entity: {:?}", current_entity);
                    input_component.left_pressed = false;
                }

                (KeyCode::ArrowRight, true) => {
                    info!("Right pressed for entity: {:?}", current_entity);
                    input_component.right_pressed = true;
                }
                (KeyCode::ArrowRight, false) => {
                    info!("Stopped pressing right for entity: {:?}", current_entity);
                    input_component.right_pressed = false;
                }

                _ => {
                    info!("Other key event: {:?}", code);
                }
            }
        }
    }
}
