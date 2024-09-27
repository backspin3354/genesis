use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

mod gfx;

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    renderer: Option<gfx::Renderer>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = WindowAttributes::default();
        let window = event_loop.create_window(attributes).unwrap();
        let window = Arc::new(window);

        let renderer = gfx::Renderer::new(window.clone());

        self.window = Some(window);
        self.renderer = Some(renderer);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.resize(new_size.width, new_size.height);
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(renderer) = self.renderer.as_mut() {
            renderer.draw();
        }
    }
}

fn main() {
    env_logger::init();

    let mut app = App::default();
    let event_loop = EventLoop::builder().build().unwrap();
    event_loop.run_app(&mut app).unwrap();
}
