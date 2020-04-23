/// Singleton state for the running application
pub struct Application {
    pub display: glium::Display,
    cursor_grabbed: bool,
    debug_overlay: bool,
}

impl Application {
    pub fn new(display: glium::Display) -> Application {
        Application {
            display,
            cursor_grabbed: false,
            debug_overlay: false,
        }
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
