use glium::glutin::dpi::LogicalSize;

/// Default settings go here

pub const RENDER_DISTANCE_U8: u8 = 2;
pub const RENDER_DISTANCE_I32: i32 = 2;

pub const VIEWPORT_WIDTH: u32 = 1024;
pub const VIEWPORT_HEIGHT: u32 = 768;

pub const VIEWPORT: LogicalSize<u32> = LogicalSize {
    width: VIEWPORT_WIDTH,
    height: VIEWPORT_HEIGHT,
};
