mod application;
mod block;
mod camera;
mod color;
mod default;
mod game;
mod game_loop;
mod instrumentation;
mod logging;
mod render;
mod space;
mod text;
mod world;
mod worldgen;

use application::Application;
use game_loop::Action;
use glium::glutin::event::ElementState::Pressed;
use glium::glutin::event::WindowEvent::{CloseRequested, KeyboardInput, Resized};
use glium::glutin::event_loop::ControlFlow;
use glium::glutin::platform::desktop::EventLoopExtDesktop;
use glium::uniform;
use glium::Surface;
use log::info;
use world::World;

fn main() {
    logging::initialize();
    let registry = instrumentation::initialize();

    let events_loop = glium::glutin::event_loop::EventLoop::new();
    let mut application = Application::new(&events_loop, registry);
    application.grab_cursor();

    run(events_loop, application)
}

fn run<T>(
    mut events_loop: glium::glutin::event_loop::EventLoop<T>,
    mut application: application::Application,
) {
    const INDICES: glium::index::NoIndices =
        glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
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
    const SKY_COLOR: (f32, f32, f32, f32) = (color::SKY[0], color::SKY[1], color::SKY[2], 1.0);

    let system = glium_text::TextSystem::new(&application.display);
    let font = glium_text::FontTexture::new(
        &application.display,
        &include_bytes!("../assets/InconsolataExpanded-Black.ttf")[..],
        70,
    )
    .unwrap();

    // metrics
    let blocks_nearby = prometheus::Gauge::new("nearby_blocks", "Blocks nearby this tick").unwrap();
    let blocks_rendered =
        prometheus::Gauge::new("rendered_blocks", "Blocks rendered this tick").unwrap();
    application.register_metric(Box::new(blocks_nearby.clone()));
    application.register_metric(Box::new(blocks_rendered.clone()));

    game_loop::run(move || {
        application.camera.update();
        let mut target = application.display.draw();
        target.clear_color_and_depth(SKY_COLOR, 1.0);
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
                    .draw(&vertices, INDICES, &program, &uniform, &params)
                    .unwrap()
            }
        }
        blocks_nearby.set(nearby_blocks_count as f64);
        blocks_rendered.set(blocks_rendered_count as f64);

        if application.get_debug_overlay() {
            let (w, h) = application.display.get_framebuffer_dimensions();

            let b_text = glium_text::TextDisplay::new(
                &system,
                &font,
                &format!("B: {}/{}", blocks_rendered.get(), blocks_nearby.get()).to_string(),
            );

            const TEXT_SIZE: f32 = 0.05;
            const HORIZONTAL_POS: f32 = -0.95;
            const VERTICAL_POS: f32 = 0.9;
            #[rustfmt::skip] // useful to be able to see each tuple on its own row
            let b_matrix:[[f32; 4]; 4] = cgmath::Matrix4::new(
                TEXT_SIZE, 0.0, 0.0, 0.0,
                0.0, TEXT_SIZE * (w as f32) / (h as f32), 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                HORIZONTAL_POS, VERTICAL_POS, 0.0, 1.0f32,
            ).into();

            glium_text::draw(
                &b_text,
                &system,
                &mut target,
                b_matrix,
                (1.0, 1.0, 1.0, 1.0),
            );

            let tps_text =
                glium_text::TextDisplay::new(&system, &font, &format!("TPS: {}", 123).to_string());
            #[rustfmt::skip] // useful to be able to see each tuple on its own row
                let tps_matrix:[[f32; 4]; 4] = cgmath::Matrix4::new(
                TEXT_SIZE, 0.0, 0.0, 0.0,
                0.0, TEXT_SIZE * (w as f32) / (h as f32), 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                HORIZONTAL_POS, VERTICAL_POS - (TEXT_SIZE + 0.02), 0.0, 1.0f32,
            ).into();

            glium_text::draw(
                &tps_text,
                &system,
                &mut target,
                tps_matrix,
                (1.0, 1.0, 1.0, 1.0),
            );
        }

        target.finish().unwrap();

        let mut action = Action::Continue;

        // polling and handling the events received by the window
        // TODO: we should use `run` instead of `run_return`
        events_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Exit;
            if let glium::glutin::event::Event::WindowEvent {
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
                                glium::glutin::event::VirtualKeyCode::Escape => {
                                    if pressed {
                                        application.toggle_cursor_grabbed()
                                    }
                                }
                                glium::glutin::event::VirtualKeyCode::F3 => {
                                    if pressed {
                                        application.toggle_debug_overlay()
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
