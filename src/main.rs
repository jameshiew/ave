#[macro_use]
extern crate glium;
extern crate glutin;
extern crate cgmath;
extern crate rand;
extern crate collision;
extern crate noise;

mod vertex;
mod block;
mod camera;
mod space;
mod color;
mod world;
mod worldgen;
mod game;

use glium::Surface;
use world::World;
use std::thread;
use std::time::{Duration, Instant};
use glutin::ElementState::Pressed;
use glutin::WindowEvent::{Closed, KeyboardInput};

/// Global, thread-safe context for the application
struct Application {
    pub display: glium::Display,
    pub camera: camera::CameraState,
    pub game: game::Game,
}

impl Application {
    pub fn new(events_loop: &glutin::EventsLoop) -> Application {
        let window = glutin::WindowBuilder::new()
            .with_dimensions(1024, 768)
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

    loop {
        match callback() {
            Action::Stop => break,
            Action::Continue => ()
        };

        let now = Instant::now();
        accumulator += now - previous_clock;
        previous_clock = now;

        let fixed_time_stamp = Duration::new(0, 16666667);
        while accumulator >= fixed_time_stamp {
            accumulator -= fixed_time_stamp;

            // if you have a game, update the state here
        }

        thread::sleep(fixed_time_stamp - accumulator);
    }
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let mut application = Application::new(&events_loop);
    application.display.gl_window().set_cursor_state(glutin::CursorState::Grab).expect("couldn't grab cursor");
    let mut cursor_grabbed = true;

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    let program = glium::Program::from_source(
        &application.display,
        include_str!("./shaders/phong.glslv"),
        include_str!("./shaders/phong.glslf"),
        None,
    ).unwrap();
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
        let perspective: [[f32; 4]; 4] = application.camera.get_perspective().into();
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
//        println!("Chunk: {}, {}, {}", cx, cy, cz);
        for x in (cx - 2)..(cx + 2) {
            for y in (cy - 2)..(cy + 2) {
                for z in (cz - 2)..(cz + 2) {
                    application.game.world.get_or_create([x, y, z].into());
                }
            }
        }

        for (position, block_type) in application.game.world.at(application.camera.position, 2) {
            if application.camera.can_see(position) {
                let vertices = block::make_cube(&application.display, &position, block_type.color);
                target.draw(
                    &vertices,
                    indices,
                    &program,
                    &uniform,
                    &params
                ).unwrap()
            }
        }

        target.finish().unwrap();

        let mut action = Action::Continue;

        // polling and handling the events received by the window
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    Closed => action = Action::Stop,
                    KeyboardInput { input, .. } => {
                        let pressed = input.state == Pressed;
                        let key = match input.virtual_keycode {
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
