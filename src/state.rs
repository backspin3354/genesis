use std::sync::Arc;

use winit::{
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::{Window, WindowAttributes},
};

mod gfx;
mod input;

pub struct AppState {
    window_arc: Arc<Window>,
    renderer: gfx::Renderer,
    input: input::Input,
}

impl AppState {
    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        let attributes = WindowAttributes::default().with_title("Genesis");
        let window = event_loop.create_window(attributes).unwrap();
        let window_arc = Arc::new(window);

        let renderer = pollster::block_on(gfx::Renderer::new(window_arc.clone()));

        let mut input = input::Input::default();

        {
            use input::Button;
            use winit::keyboard::KeyCode;

            input.create_action_and_binding("move_forward", Button::Key(KeyCode::KeyW));
            input.create_action_and_binding("move_back", Button::Key(KeyCode::KeyS));
            input.create_action_and_binding("move_left", Button::Key(KeyCode::KeyA));
            input.create_action_and_binding("move_right", Button::Key(KeyCode::KeyD));
        }

        Self {
            window_arc,
            renderer,
            input,
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
                self.renderer.resize(new_size.width, new_size.height);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                // Pass valid key events to `input`.
                if let PhysicalKey::Code(keycode) = event.physical_key {
                    self.input
                        .update_button(&input::Button::Key(keycode), event.state.is_pressed());
                }
            }
            _ => {}
        }
    }
}
