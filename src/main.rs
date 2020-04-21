mod application;
mod block;
mod camera;
mod color;
mod default;
mod event_loop;
mod game;
mod render;
mod space;
mod world;
mod worldgen;

use application::Application;
use event_loop::{run, Action};
use glium::uniform;
use glium::Surface;
use glium::glutin::ElementState::Pressed;
use glium::glutin::WindowEvent::{CloseRequested, KeyboardInput, Resized};
use log::debug;
use log::info;
use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode};
use world::World;

fn main() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Stdout,
    )
    .unwrap()])
    .unwrap();
    let mut events_loop = glium::glutin::EventsLoop::new();
    let mut application = Application::new(&events_loop);
    application
        .display
        .gl_window()
        .window()
        .grab_cursor(true)
        .expect("couldn't grab cursor");
    application.display.gl_window().window().hide_cursor(true);
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

    run(move || {
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
        for (position, block_type) in application
            .game
            .world
            .at(application.camera.position, default::RENDER_DISTANCE_U8)
        {
            nearby_blocks_count += 1;
            if application.camera.can_see(position) {
                blocks_rendered_count += 1;
                let vertices = block::make_cube(
                    &application.display,
                    &position,
                    block_type.color,
                    block::Mask::new(),
                );
                target
                    .draw(&vertices, indices, &program, &uniform, &params)
                    .unwrap()
            }
        }
        debug!(
            "{} blocks rendered of {} blocks nearby",
            blocks_rendered_count, nearby_blocks_count
        );

        target.finish().unwrap();

        let mut action = Action::Continue;

        // polling and handling the events received by the window
        events_loop.poll_events(|event| {
            if let glium::glutin::Event::WindowEvent {
                event: window_event,
                ..
            } = event
            {
                match window_event {
                    CloseRequested => action = Action::Stop,
                    Resized(new) => {
                        info!("Window resized to {}px x {}px", new.width, new.height);
                    }
                    KeyboardInput { input, .. } => {
                        let pressed = input.state == Pressed;
                        if let Some(key) = input.virtual_keycode {
                            match key {
                                glium::glutin::VirtualKeyCode::Escape => {
                                    if pressed {
                                        if cursor_grabbed {
                                            application
                                                .display
                                                .gl_window()
                                                .window()
                                                .hide_cursor(false);
                                            application
                                                .display
                                                .gl_window()
                                                .window()
                                                .grab_cursor(false)
                                                .expect("couldn't ungrab cursor");
                                            cursor_grabbed = false;
                                        } else {
                                            application
                                                .display
                                                .gl_window()
                                                .window()
                                                .grab_cursor(true)
                                                .expect("couldn't grab cursor");
                                            application
                                                .display
                                                .gl_window()
                                                .window()
                                                .hide_cursor(true);
                                            cursor_grabbed = true;
                                        }
                                    }
                                }
                                _ => application.camera.process_input(pressed, key),
                            }
                        }
                    }
                    _ => (),
                }
            }
        });

        action
    });
}
