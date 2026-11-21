use std::sync::Arc;

use softbuffer::{Context, Surface};
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Transform};
use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent, window::Window,
};

use crate::flock::Flock;

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    graphics: Option<GraphicsContext>,
    flock: Flock,
}

struct GraphicsContext {
    context: Context<Arc<Window>>,
    surface: Surface<Arc<Window>, Arc<Window>>,
}

impl GraphicsContext {
    fn new(context: Context<Arc<Window>>, surface: Surface<Arc<Window>, Arc<Window>>) -> Self {
        Self { context, surface }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("Boids")
                .with_inner_size(LogicalSize::new(800, 600));

            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

            let context = softbuffer::Context::new(window.clone()).unwrap();
            let mut surface = softbuffer::Surface::new(&context.clone(), window.clone()).unwrap();

            let size = window.inner_size();
            if size.width > 0 && size.height > 0 {
                surface
                    .resize(
                        std::num::NonZeroU32::new(size.width).unwrap(),
                        std::num::NonZeroU32::new(size.height).unwrap(),
                    )
                    .unwrap();
            }

            self.graphics = Some(GraphicsContext::new(context, surface));
            self.window = Some(window);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let window = match &self.window {
            Some(win) if win.id() == window_id => win,
            _ => return,
        };

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(graphics) = &mut self.graphics
                    && size.width > 0
                    && size.height > 0
                {
                    graphics
                        .surface
                        .resize(
                            std::num::NonZeroU32::new(size.width).unwrap(),
                            std::num::NonZeroU32::new(size.height).unwrap(),
                        )
                        .unwrap();
                }
            }
            WindowEvent::RedrawRequested => {
                let Some(graphics) = &mut self.graphics else {
                    return;
                };
                let size = window.inner_size();

                if size.width == 0 || size.height == 0 {
                    return;
                }

                self.flock.update();
                let mut pixmap = Pixmap::new(size.width, size.height).unwrap();
                pixmap.fill(Color::TRANSPARENT);

                for boid in self.flock.boids() {
                    let mut pb = PathBuilder::new();
                    pb.push_circle(boid.x, boid.y, 2.0);
                    let circle_path = pb.finish().unwrap();

                    let mut paint = Paint::default();
                    paint.set_color_rgba8(255, 0, 0, 255);

                    pixmap.fill_path(
                        &circle_path,
                        &paint,
                        FillRule::Winding,
                        Transform::identity(),
                        None,
                    );
                }

                let mut buffer = graphics.surface.buffer_mut().unwrap();

                let pixels = pixmap.data();

                for (chunk, i) in pixels.as_chunks::<4>().0.iter().zip(0..) {
                    let r = chunk[0] as u32;
                    let g = chunk[1] as u32;
                    let b = chunk[2] as u32;
                    buffer[i] = (r << 16) | (g << 8) | b;
                }

                buffer.present().unwrap();

                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
