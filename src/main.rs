#[macro_use]
extern crate glium;
extern crate image;

mod support;
mod vertex;
mod block;

use glium::{glutin, Surface};
use vertex::Vertex;
use block::Chunk;
use block::BlockType;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new()
        .with_depth_buffer(24)
        .with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let program = glium::Program::from_source(
        &display,
        include_str!("./shaders/notexture.glslv"),
        include_str!("./shaders/notexture.glslf"),
        None,
    ).unwrap();

    let mut camera = support::camera::CameraState::new();

    let model = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ];

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let light = [1.4, 0.4, 0.7f32];

    implement_vertex!(Vertex, position, normal);

    let mut chunk = Chunk::new(0, 0, 0);
    for z in 0..32 {
        chunk.set((1, 1, z), BlockType::Solid);
        chunk.set((2, (z * 2 + 1) % 32, 10), BlockType::Solid);
    }

    support::start_loop(|| {
        camera.update();
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        for x in 0..32 {
            for y in 0..32 {
                for z in 0..32 {
                    match chunk.get_vertices(&display, x, y, z) {
                        Some(block) => target.draw(
                            &block,
                            glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                            &program,
                            &uniform! {
                                model: model,
                                perspective: camera.get_perspective(),
                                view: camera.get_view(),
                                u_light: light,
                            },
                            &params
                        ).unwrap(),
                        None => ()
                    }
                }
            }
        }
        target.finish().unwrap();

        let mut action = support::Action::Continue;

        // polling and handling the events received by the window
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => action = support::Action::Stop,
                    ev => camera.process_input(&ev),
                },
                _ => (),
            }
        });

        return action;
    });
}


fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}
