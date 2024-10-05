use std::sync::Arc;

use glam::{Vec2, Vec3};
use input::Button;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, DeviceId, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{CursorGrabMode, Window, WindowAttributes, WindowId},
};

mod gfx;
mod input;

#[derive(Default)]
struct App {
    input: input::Input,
    window: Option<Arc<Window>>,
    renderer: Option<gfx::Renderer>,
    camera: gfx::Camera,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = WindowAttributes::default();
        let window = event_loop.create_window(attributes).unwrap();
        let window = Arc::new(window);

        if window
            .set_cursor_grab(CursorGrabMode::Locked)
            .or_else(|_| window.set_cursor_grab(CursorGrabMode::Confined))
            .is_err()
        {
            log::warn!("Couldn't grab cursor");
        } else {
            window.set_cursor_visible(false);
        }

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
        renderer.load_billboards(&[
            gfx::Billboard {
                position: glam::Vec3::new(0.0, 0.0, 0.0),
                color: glam::Vec3::X,
            },
            gfx::Billboard {
                position: glam::Vec3::new(4.0, 0.0, 4.0),
                color: glam::Vec3::Y,
            },
            gfx::Billboard {
                position: glam::Vec3::new(2.0, 0.0, 8.0),
                color: glam::Vec3::Z,
            },
        ]);

        self.window = Some(window);
        self.renderer = Some(renderer);

        self.input.register_action("move_forward");
        self.input
            .bind_button(Button::Key(KeyCode::KeyW), "move_forward");

        self.input.register_action("move_back");
        self.input
            .bind_button(Button::Key(KeyCode::KeyS), "move_back");

        self.input.register_action("move_left");
        self.input
            .bind_button(Button::Key(KeyCode::KeyA), "move_left");

        self.input.register_action("move_right");
        self.input
            .bind_button(Button::Key(KeyCode::KeyD), "move_right");

        self.input.register_action("run");
        self.input
            .bind_button(Button::Key(KeyCode::ShiftLeft), "run");

        self.input.register_action("use");
        self.input
            .bind_button(Button::Mouse(MouseButton::Left), "use");
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.resize(new_size.width, new_size.height);
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    self.input
                        .update_button(Button::Key(code), event.state.is_pressed());
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.input
                    .update_button(Button::Mouse(button), state.is_pressed());
            }
            _ => {}
        }
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => self
                .input
                .update_mouse_delta(Vec2::new(delta.0 as f32, delta.1 as f32)),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        // TODO: Remove placeholder camera controller
        {
            let delta = 1.0 / 60.0;

            let mouse_delta = self.input.get_mouse_delta() * 0.01;
            self.camera.rotation.x -= mouse_delta.x;
            self.camera.rotation.y -= mouse_delta.y;

            let mut velocity = Vec3::ZERO;

            let (sin_yaw, cos_yaw) = self.camera.rotation.x.sin_cos();
            let forward = Vec3::new(cos_yaw, 0.0, sin_yaw);
            let right = Vec3::new(sin_yaw, 0.0, -cos_yaw);

            if self.input.get_action("move_forward").is_down {
                velocity += forward;
            }
            if self.input.get_action("move_back").is_down {
                velocity -= forward;
            }
            if self.input.get_action("move_left").is_down {
                velocity -= right;
            }
            if self.input.get_action("move_right").is_down {
                velocity += right;
            }

            self.camera.position += velocity
                * delta
                * if self.input.get_action("run").is_down {
                    1.5
                } else {
                    1.0
                };
        }

        if let Some(renderer) = self.renderer.as_mut() {
            renderer.load_camera(&self.camera);
            renderer.draw();
        }

        self.input.update();
    }

    fn exiting(&mut self, _: &ActiveEventLoop) {
        self.renderer = None;
    }
}

fn main() {
    env_logger::init();
    log::info!("Hello!");

    let mut app = App::default();
    let event_loop = EventLoop::builder().build().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();

    log::info!("Goodbye!");
}
