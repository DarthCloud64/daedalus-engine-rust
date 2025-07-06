use dotenv::dotenv;
use log::info;
use winit::event_loop::{ControlFlow, EventLoop};

use crate::application::Application;

mod application;
mod ecs;
mod input;
mod rendering;

#[pollster::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    info!("Starting Daedalus Engine...");

    let event_loop = EventLoop::new().expect("Failed to create event loop");

    // Configure the event loop to constantly run
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app: Application = Application::new(800, 600);
    let _ = event_loop
        .run_app(&mut app)
        .expect("Failed to run application");
}
