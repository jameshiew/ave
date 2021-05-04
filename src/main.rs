mod application;
mod block;
mod camera;
mod color;
mod default;
mod game;
mod render;
mod space;
mod world;
mod world_renderer;
mod worldgen;

use crate::game::Ticker;
use application::Application;
use glium::glutin::event::ElementState::Pressed;
use glium::glutin::event::WindowEvent::{CloseRequested, KeyboardInput, Resized};
use glium::glutin::event_loop::ControlFlow;
use glium::glutin::platform::run_return::EventLoopExtRunReturn;
use glium::Surface;
use log::info;

const TITLE: &str = "Ave";

fn main() {
    env_logger::init();

    let game = game::Game::new();

    let window = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(default::VIEWPORT)
        .with_title(TITLE);
    let context = glium::glutin::ContextBuilder::new()
        .with_depth_buffer(24)
        .with_vsync(true);
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let display = glium::Display::new(window, context, &event_loop).unwrap();
    let mut application = Application::new(display);
    application.grab_cursor();

    run(event_loop, application, game)
}

fn run<T>(
    mut events_loop: glium::glutin::event_loop::EventLoop<T>,
    mut application: application::Application,
    mut game: game::Game,
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

    let world_renderer = world_renderer::WorldRenderer::new(INDICES, program, params);

    const SKY_COLOR: (f32, f32, f32, f32) = (color::SKY[0], color::SKY[1], color::SKY[2], 1.0);

    let system = glium_text_rusttype::TextSystem::new(&application.display);
    let font = glium_text_rusttype::FontTexture::new(
        &application.display,
        &include_bytes!("../assets/InconsolataExpanded-Black.ttf")[..],
        70,
        glium_text_rusttype::FontTexture::ascii_character_list(),
    )
    .unwrap();

    let ticker = Ticker::new();
    ticker.run(|| {
        game.tick();

        let mut target = application.display.draw();
        target.clear_color_and_depth(SKY_COLOR, 1.0);

        world_renderer.render(&game, &application.display, &mut target);

        if application.get_debug_overlay() {
            let (w, h) = application.display.get_framebuffer_dimensions();

            let b_text = glium_text_rusttype::TextDisplay::new(
                &system,
                &font,
                &format!(
                    "B: {}/{}",
                    world_renderer.get_blocks_rendered(),
                    world_renderer.get_blocks_nearby()
                ),
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

            glium_text_rusttype::draw(
                &b_text,
                &system,
                &mut target,
                b_matrix,
                (1.0, 1.0, 1.0, 1.0),
            )
            .unwrap();

            let tps_text = glium_text_rusttype::TextDisplay::new(
                &system,
                &font,
                &format!("TPS: {}", ticker.get_tps()),
            );
            #[rustfmt::skip] // useful to be able to see each tuple on its own row
                let tps_matrix:[[f32; 4]; 4] = cgmath::Matrix4::new(
                TEXT_SIZE, 0.0, 0.0, 0.0,
                0.0, TEXT_SIZE * (w as f32) / (h as f32), 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                HORIZONTAL_POS, VERTICAL_POS - (TEXT_SIZE + 0.02), 0.0, 1.0f32,
            ).into();

            glium_text_rusttype::draw(
                &tps_text,
                &system,
                &mut target,
                tps_matrix,
                (1.0, 1.0, 1.0, 1.0),
            )
            .unwrap();
        }

        target.finish().unwrap();

        let mut should_continue = true;

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
                    CloseRequested => should_continue = false,
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
                                _ => game.camera.process_input(pressed, key),
                            }
                        }
                    }
                    _ => (),
                }
            }
        });

        should_continue
    });
}
