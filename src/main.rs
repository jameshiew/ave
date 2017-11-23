#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;
extern crate genmesh;
extern crate obj;

mod vertex;
mod block;
mod camera;
mod space;

use glium::{glutin, Surface};
use cgmath::Matrix4;
use block::{BlockType, World};
use std::thread;
use std::time::{Duration, Instant};

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
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new()
        .with_depth_buffer(24)
        .with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let program = glium::Program::from_source(
        &display,
        include_str!("./shaders/simple.glslv"),
        include_str!("./shaders/simple.glslf"),
        None,
    ).unwrap();

    let mut camera = camera::CameraState::new();

    let model: [[f32; 4]; 4] = Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0f32,
    ).into();

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullingDisabled,
        polygon_mode: glium::draw_parameters::PolygonMode::Fill,
        smooth: Some(glium::draw_parameters::Smooth::Nicest),
        ..Default::default()
    };

    let mut world = World::new();

    start_loop(|| {
        camera.update();
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        for x in 0..1 {
            for y in -1..2 {
                for z in 0..1 {
                    let chunk = world.get_or_create(x, y, z);
                    for (chunk_position, block_type) in chunk.get_visible() {
                        match block_type {
                            &BlockType::Solid => {
                                let world_position = World::get_position(x, y, z, chunk_position.0, chunk_position.1, chunk_position.2);
                                let vertices = block::make_cube(&display, world_position.0, world_position.1, world_position.2);
                                target.draw(
                                    &vertices,
                                    glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                                    &program,
                                    &uniform! {
                                model: model,
                                perspective: camera.get_perspective(),
                                view: camera.get_view(),
                            },
                                    &params
                                ).unwrap()
                            },
                            &BlockType::Empty => (),
                        }
                    }
                }
            }
        }
        target.finish().unwrap();

        let mut action = Action::Continue;

        // polling and handling the events received by the window
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => action = Action::Stop,
                    ev => camera.process_input(&ev),
                },
                _ => (),
            }
        });

        return action;
    });
}
