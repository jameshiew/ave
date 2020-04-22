use crate::{camera, default, game};

/// Singleton state for the running application
pub struct Application {
    pub display: glium::Display,
    pub camera: camera::CameraState,
    pub game: game::Game,
    cursor_grabbed: bool,
}

impl Application {
    pub fn new(events_loop: &glium::glutin::EventsLoop) -> Application {
        let window = glium::glutin::WindowBuilder::new()
            .with_dimensions(default::VIEWPORT)
            .with_title("Ave");
        let context = glium::glutin::ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(true);
        let display = glium::Display::new(window, context, events_loop).unwrap();
        let camera = camera::CameraState::new();
        let game = game::Game::new();
        Application {
            display,
            camera,
            game,
            cursor_grabbed: false,
        }
    }
    pub fn grab_cursor(&mut self) {
        self.display
            .gl_window()
            .window()
            .grab_cursor(true)
            .expect("couldn't grab cursor");
        self.display.gl_window().window().hide_cursor(true);
        self.cursor_grabbed = true;
    }
    pub fn ungrab_cursor(&mut self) {
        self.display.gl_window().window().hide_cursor(false);
        self.display
            .gl_window()
            .window()
            .grab_cursor(false)
            .expect("couldn't ungrab cursor");
        self.cursor_grabbed = false;
    }
    pub fn toggle_cursor_grabbed(&mut self) {
        if self.cursor_grabbed {
            self.ungrab_cursor()
        } else {
            self.grab_cursor()
        }
    }
}
