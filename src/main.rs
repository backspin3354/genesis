use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

mod gfx;

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    renderer: Option<gfx::Renderer>,
    camera: gfx::Camera,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = WindowAttributes::default();
        let window = event_loop.create_window(attributes).unwrap();
        let window = Arc::new(window);

        let mut renderer = gfx::Renderer::new(window.clone());
        renderer.load_mesh(
            &[
                gfx::Vertex {
                    position: glam::Vec3::new(0.0, 0.5, 4.0),
                    color: glam::Vec3::new(1.0, 0.0, 0.0),
                },
                gfx::Vertex {
                    position: glam::Vec3::new(0.5, -0.5, 4.0),
                    color: glam::Vec3::new(0.0, 1.0, 0.0),
                },
                gfx::Vertex {
                    position: glam::Vec3::new(-0.5, -0.5, 4.0),
                    color: glam::Vec3::new(0.0, 0.0, 1.0),
                },
            ],
            &[0, 1, 2, 0],
        );

        self.window = Some(window);
        self.renderer = Some(renderer);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
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
        self.camera.rotation.x += 0.01;
        self.camera.rotation.y += 0.003;

        if let Some(renderer) = self.renderer.as_mut() {
            renderer.load_camera(&self.camera);
            renderer.draw();
        }
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        self.renderer = None;
    }
}

fn main() {
    env_logger::init();

    let mut app = App::default();
    let event_loop = EventLoop::builder().build().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();
}
