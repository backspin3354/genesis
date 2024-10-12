use std::sync::Arc;

use winit::{
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

mod gfx;

pub struct AppState {
    window_arc: Arc<Window>,
    renderer: gfx::Renderer,
}

impl AppState {
    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        let attributes = WindowAttributes::default();
        let window = event_loop.create_window(attributes).unwrap();
        let window_arc = Arc::new(window);

        let renderer = pollster::block_on(gfx::Renderer::new(window_arc.clone()));

        Self {
            window_arc,
            renderer,
        }
    }

    pub fn update(&mut self, event_loop: &ActiveEventLoop) {
        self.renderer.render();
    }
}

impl AppState {
    pub fn window_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                self.window_arc.set_title(&format!("Hello {:?}", new_size));
                self.renderer.resize(new_size.width, new_size.height);
            }
            _ => {}
        }
    }
}
