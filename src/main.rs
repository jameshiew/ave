#[macro_use]
extern crate glium;
extern crate glutin;
extern crate cgmath;
extern crate rand;
extern crate collision;
extern crate noise;
#[macro_use]
extern crate log;
extern crate simplelog;

mod render;
mod block;
mod camera;
mod space;
mod color;
mod world;
mod worldgen;
mod game;
mod default;

use glium::Surface;
use world::World;
use std::thread;
use std::time::{Duration, Instant};
use glutin::ElementState::Pressed;
use glutin::WindowEvent::{Closed, Resized, KeyboardInput};

use simplelog::{Config, TermLogger, CombinedLogger, LogLevelFilter};

/// Global, thread-safe context for the application
struct Application {
    pub display: glium::Display,
    pub camera: camera::CameraState,
    pub game: game::Game,
}

impl Application {
    pub fn new(events_loop: &glutin::EventsLoop) -> Application {
        let window = glutin::WindowBuilder::new()
            .with_dimensions(default::VIEWPORT_WIDTH, default::VIEWPORT_HEIGHT)
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

pub enum Action {
    Stop,
    Continue,
}

pub fn start_loop<F>(mut callback: F) where F: FnMut() -> Action {
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();

    // not really sure how to measure FPS accurately so call it TPS
    let mut ticks_per_second;
    let mut this_second = Duration::new(0, 0);
    let mut ticks_this_second = 0;

    loop {
        match callback() {
            Action::Stop => break,
            Action::Continue => ()
        };
        ticks_this_second += 1;

        let now = Instant::now();
        let time_passed = now - previous_clock;
        previous_clock = now;

        this_second += time_passed;
        if this_second > Duration::new(1, 0) {
            ticks_per_second = ticks_this_second;
            ticks_this_second = 0;
            this_second = Duration::new(0, 0);
            debug!("TPS: {}", ticks_per_second)
        }

        accumulator += time_passed;

        let fixed_time_stamp = Duration::new(0, 16666667);
        while accumulator >= fixed_time_stamp {
            accumulator -= fixed_time_stamp;

            // if you have a game, update the state here
        }

        thread::sleep(fixed_time_stamp - accumulator);
    }
}

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LogLevelFilter::Debug, Config::default()).unwrap(),
        ]
    ).unwrap();
    let mut events_loop = glutin::EventsLoop::new();
    let mut application = Application::new(&events_loop);
    application.display.gl_window().set_cursor_state(glutin::CursorState::Grab).expect("couldn't grab cursor");
    let mut cursor_grabbed = true;

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    let program = render::get_shader(&application.display, render::Shaders::Phong);
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        polygon_mode: glium::draw_parameters::PolygonMode::Fill,
        smooth: Some(glium::draw_parameters::Smooth::Nicest),
        ..Default::default()
    };
    let sky_color = (color::SKY[0], color::SKY[1], color::SKY[2], 1.0);

    start_loop(move || {
        application.camera.update();
        let mut target = application.display.draw();
        target.clear_color_and_depth(sky_color, 1.0);
        let perspective: [[f32; 4]; 4] = application.camera.perspective.into();
        let view: [[f32; 4]; 4] = application.camera.get_view().into();
        let uniform = uniform! {
            model: space::MODEL,
            perspective: perspective,
            view: view
        };

        // generate chunks as we move the camera
        let chunk_coords = world::position_to_chunk(&application.camera.position);
        let cx = chunk_coords.x;
        let cy = chunk_coords.y;
        let cz = chunk_coords.z;
        for x in (cx - default::RENDER_DISTANCE_I32)..(cx + default::RENDER_DISTANCE_I32) {
            for y in (cy - default::RENDER_DISTANCE_I32)..(cy + default::RENDER_DISTANCE_I32) {
                for z in (cz - default::RENDER_DISTANCE_I32)..(cz + default::RENDER_DISTANCE_I32) {
                    application.game.world.get_or_create([x, y, z].into());
                }
            }
        }

        let mut nearby_blocks_count = 0;
        let mut blocks_rendered_count = 0;
        for (position, block_type) in application.game.world.at(application.camera.position, default::RENDER_DISTANCE_U8) {
            nearby_blocks_count += 1;
            if application.camera.can_see(position) {
                blocks_rendered_count += 1;
                let vertices = block::make_cube(&application.display, &position, block_type.color, block::Mask::new());
                target.draw(
                    &vertices,
                    indices,
                    &program,
                    &uniform,
                    &params
                ).unwrap()
            }
        }
        debug!("{} blocks rendered of {} blocks nearby", blocks_rendered_count, nearby_blocks_count);

        target.finish().unwrap();

        let mut action = Action::Continue;

        // polling and handling the events received by the window
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    Closed => action = Action::Stop,
                    Resized(w, h) => {
                        info!("Window resized to {}px x {}px", w, h);
                    },
                    KeyboardInput { input, .. } => {
                        let pressed = input.state == Pressed;
                        match input.virtual_keycode {
                            Some(key) => match key {
                                glutin::VirtualKeyCode::Escape => {
                                    if pressed {
                                        if cursor_grabbed {
                                            application.display.gl_window().set_cursor_state(glutin::CursorState::Normal).expect("couldn't ungrab cursor");
                                            cursor_grabbed = false;
                                        } else {
                                            application.display.gl_window().set_cursor_state(glutin::CursorState::Grab).expect("couldn't grab cursor");
                                            cursor_grabbed = true;
                                        }
                                    }
                                }
                                _ => application.camera.process_input(pressed, key),
                            },
                            None => (),
                        };
                    },
                    _ => (),
                },
                _ => (),
            }
        });

        return action;
    });
}
