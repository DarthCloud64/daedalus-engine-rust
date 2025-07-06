use glam::Vec3;
use log::{info, trace, warn};
use std::sync::Arc;
use wgpu::SurfaceError;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::{Window, WindowAttributes},
};

use crate::{
    ecs::{
        component::{
            camera::CameraComponent, input::InputComponent, transform::TransformComponent,
        },
        entity::scene::Scene,
    },
    input::InputService,
    rendering::RenderingService,
};

#[derive(Default)]
pub struct Application {
    width: i32,
    height: i32,
    window: Option<Arc<Window>>,
    scene: Option<Scene>,
    rendering_service: Option<RenderingService>,
    input_service: Option<InputService>,
}

impl Application {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            window: None,
            scene: None,
            rendering_service: None,
            input_service: None,
        }
    }
    fn setup_scene(&mut self) {
        self.scene = Some(Scene::new());
        let scene = self.scene.as_mut().unwrap();

        // TODO: this is a test entity, remove later
        let test_entity = scene.create_entity();
        scene
            .input_components
            .insert(test_entity, InputComponent::default());
        scene.camera_components.insert(
            test_entity,
            CameraComponent {
                look_at: Vec3::new(0.0, 0.0, 0.0), // Looking at the origin
                up_orientation: Vec3::Y,           // Up is the positive Y direction
                aspect_ratio: self.width as f32 / self.height as f32,
                field_of_view: 45.0,
                z_near_field: 0.1,
                z_far_field: 100.0,
            },
        );
        scene.transform_components.insert(
            test_entity,
            TransformComponent {
                // +Z is out of the screen
                position: Vec3::new(0.0, 0.0, 2.0),
                scale: Vec3::new(1.0, 1.0, 1.0), // Scale of 1 means no scaling
                rotation: Vec3::new(0.0, 0.0, 0.0), // No rotation
                translation: Vec3::new(0.0, 0.0, 0.0), // No translation
            },
        );
    }

    fn update_services(&self) {
        todo!("Update services as needed");
    }

    fn present(&mut self) {
        trace!("Presenting frame...");
        let rendering_service = self.rendering_service.as_mut().unwrap();
        match rendering_service.render() {
            Ok(_) => {
                trace!("Presented frame!");
            }
            Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                let size = self.window.as_ref().unwrap().inner_size();
                warn!("Surface lost or outdated, resizing...{:?}", size);
                self.rendering_service
                    .as_mut()
                    .unwrap()
                    .resize_surface(size.width, size.height);
            }
            Err(e) => {
                warn!("Rendering error: {:?}", e);
            }
        }
    }
}

impl ApplicationHandler for Application {
    // This effectively initializes the application including the services
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("Daedalus Engine")
                        .with_inner_size(LogicalSize::new(self.width, self.height)),
                )
                .unwrap(),
        ));

        self.setup_scene();

        let main_camera_component = self
            .scene
            .as_ref()
            .unwrap()
            .camera_components
            .get(&1)
            .unwrap();

        let main_transform_component = self
            .scene
            .as_ref()
            .unwrap()
            .transform_components
            .get(&1)
            .unwrap();

        self.rendering_service = Some(
            pollster::block_on(RenderingService::new(
                self.window.as_ref().unwrap().clone(),
                main_camera_component,
                main_transform_component,
            ))
            .unwrap(),
        );

        self.input_service = Some(InputService::new());

        self.rendering_service.as_mut().unwrap().resize_surface(
            self.window.as_ref().unwrap().inner_size().width,
            self.window.as_ref().unwrap().inner_size().height,
        );
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                info!("Close window requested");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                trace!("Redraw requested for window: {:?}", window_id);
                self.present();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => {
                self.input_service.as_ref().unwrap().handle_input(
                    event_loop,
                    code,
                    key_state.is_pressed(),
                    self.scene.as_mut().unwrap(),
                );
            }
            _ => {}
        }
    }
}
