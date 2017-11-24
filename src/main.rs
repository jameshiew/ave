#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;
extern crate genmesh;
extern crate obj;
extern crate rand;

mod vertex;
mod block;
mod camera;
mod space;
mod color;
mod world;
mod worldgen;

use glium::{glutin, Surface};
use cgmath::Matrix4;
use world::{World, InMemoryWorld};
use std::thread;
use std::time::{Duration, Instant};

struct Game {
    pub world: InMemoryWorld,
}

impl Game {
    fn new() -> Game {
        Game { world: InMemoryWorld::new() }
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
    let window = glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title("Ave");
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
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        polygon_mode: glium::draw_parameters::PolygonMode::Fill,
        smooth: Some(glium::draw_parameters::Smooth::Nicest),
        ..Default::default()
    };

    let mut game = Box::new(Game::new());

    for x in -2..2 {
        for y in -2..2 {
            for z in -2..2 {
                game.world.get_or_create([x, y, z].into());
            }
        }
    }
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let sky = (color::SKY[0], color::SKY[1], color::SKY[2], 1.0);
    start_loop(|| {
        camera.update();
        let mut target = display.draw();
        target.clear_color_and_depth(sky, 1.0);

        for (position, block_type) in game.world.at(camera.position, 1) {
            let vertices = block::make_cube(&display, &position, block_type.color);
            target.draw(
                &vertices,
                indices,
                &program,
                &uniform! {
                    model: model,
                    perspective: camera.get_perspective(),
                    view: camera.get_view(),
                },
                &params
            ).unwrap()
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
