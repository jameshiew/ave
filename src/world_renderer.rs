use crate::world::World;
use crate::{block, default, game, space};

use glium::uniform;
use glium::Surface;

pub struct WorldRenderer<'a> {
    blocks_nearby: prometheus::Gauge,
    blocks_rendered: prometheus::Gauge,

    indices: glium::index::NoIndices,
    program: glium::Program,
    draw_params: glium::DrawParameters<'a>,
}

impl WorldRenderer<'_> {
    pub fn new(
        indices: glium::index::NoIndices,
        program: glium::Program,
        draw_params: glium::DrawParameters,
    ) -> WorldRenderer {
        WorldRenderer {
            blocks_nearby: prometheus::Gauge::new("nearby_blocks", "Blocks nearby this tick")
                .unwrap(),
            blocks_rendered: prometheus::Gauge::new("rendered_blocks", "Blocks rendered this tick")
                .unwrap(),

            indices,
            program,
            draw_params,
        }
    }
    pub fn render(&self, game: &game::Game, display: &glium::Display, target: &mut glium::Frame) {
        let perspective: [[f32; 4]; 4] = game.camera.perspective.into();
        let view: [[f32; 4]; 4] = game.camera.get_view().into();
        let uniform = uniform! {
            model: space::MODEL,
            perspective: perspective,  // TODO: can I inline perspective + view?
            view: view
        };

        let mut nearby_blocks_count = 0;
        let mut blocks_rendered_count = 0;
        for (position, block_type) in game
            .world
            .at(game.camera.position, default::RENDER_DISTANCE_U8)
        {
            nearby_blocks_count += 1;
            if game.camera.can_see(position) {
                blocks_rendered_count += 1;
                let vertices =
                    block::make_cube(display, &position, block_type.color, block::Mask::new());
                target
                    .draw(
                        &vertices,
                        self.indices,
                        &self.program,
                        &uniform,
                        &self.draw_params,
                    )
                    .unwrap()
            }
        }
        self.blocks_nearby.set(nearby_blocks_count as f64);
        self.blocks_rendered.set(blocks_rendered_count as f64);
    }
    pub fn get_blocks_rendered(&self) -> f64 {
        self.blocks_rendered.get()
    }
    pub fn get_blocks_nearby(&self) -> f64 {
        self.blocks_nearby.get()
    }
}
