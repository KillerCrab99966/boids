use winit::event_loop::{ControlFlow, EventLoop};

use crate::app::App;

mod app;
mod boid;
mod flock;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();

    event_loop.run_app(&mut app)?;

    Ok(())
}
