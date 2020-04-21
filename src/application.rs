use crate::{camera, default, game};

/// Global, thread-safe context for the application
pub struct Application {
    pub display: glium::Display,
    pub camera: camera::CameraState,
    pub game: game::Game,
}

impl Application {
    pub fn new(events_loop: &glutin::EventsLoop) -> Application {
        let window = glutin::WindowBuilder::new()
            .with_dimensions(default::VIEWPORT)
            .with_title("Ave");
        let context = glutin::ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(true);
        let display = glium::Display::new(window, context, events_loop).unwrap();
        let camera = camera::CameraState::new();
        let game = game::Game::new();
        Application {
            display,
            camera,
            game,
        }
    }
}
