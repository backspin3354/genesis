use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

mod state;

#[derive(Default)]
enum App {
    #[default]
    Uninitialized,
    Initialized(state::AppState),
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        match self {
            Self::Uninitialized => {
                let state = state::AppState::new(event_loop);

                *self = Self::Initialized(state);

                log::info!("Initialized");

                event_loop.set_control_flow(ControlFlow::Poll);
            }
            Self::Initialized(_) => {
                todo!();
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match self {
            Self::Uninitialized => {}
            Self::Initialized(state) => state.window_event(event_loop, event),
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        match self {
            Self::Uninitialized => {}
            Self::Initialized(state) => state.update(event_loop),
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        log::info!("Exiting");
    }
}

fn main() {
    env_logger::init();

    let mut app = App::default();
    let event_loop = EventLoop::builder().build().unwrap();
    event_loop.run_app(&mut app).unwrap();
}
