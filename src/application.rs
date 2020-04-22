use crate::{camera, default, game};

/// Singleton state for the running application
pub struct Application {
    pub registry: prometheus::Registry,
    pub display: glium::Display,
    pub camera: camera::CameraState,
    pub game: game::Game,
    cursor_grabbed: bool,
    debug_overlay: bool,
}

impl Application {
    pub fn new<T>(
        events_loop: &glium::glutin::event_loop::EventLoop<T>,
        registry: prometheus::Registry,
    ) -> Application {
        let window = glium::glutin::window::WindowBuilder::new()
            .with_inner_size(default::VIEWPORT)
            .with_title("Ave");
        let context = glium::glutin::ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(true);
        let display = glium::Display::new(window, context, events_loop).unwrap();
        let camera = camera::CameraState::new();
        let game = game::Game::new();

        Application {
            registry,
            display,
            camera,
            game,
            cursor_grabbed: false,
            debug_overlay: false,
        }
    }
    pub fn register_metric(&mut self, c: Box<dyn prometheus::core::Collector>) {
        self.registry.register(c).unwrap();
    }
    pub fn toggle_debug_overlay(&mut self) {
        self.debug_overlay = !self.debug_overlay;
    }
    pub fn get_debug_overlay(&self) -> bool {
        self.debug_overlay
    }
    pub fn grab_cursor(&mut self) {
        self.display
            .gl_window()
            .window()
            .set_cursor_grab(true)
            .expect("couldn't grab cursor");
        self.display.gl_window().window().set_cursor_visible(false);
        self.cursor_grabbed = true;
    }
    pub fn ungrab_cursor(&mut self) {
        self.display
            .gl_window()
            .window()
            .set_cursor_grab(false)
            .expect("couldn't ungrab cursor");
        self.display.gl_window().window().set_cursor_visible(true);
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
