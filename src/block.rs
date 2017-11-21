extern crate glium;

use vertex::Vertex;

use glium::vertex::VertexBuffer;
use glium::backend::Facade;

pub struct Block {
    x: f32,
    y: f32,
    z: f32,
}

impl Block {
    pub fn new() -> Block {
        Block {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn get_vertices<F: ?Sized>(self, facade: &F) -> VertexBuffer<Vertex> where F: Facade {
        return VertexBuffer::new(facade, &[
            Vertex { position: [-1.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0] },
            Vertex { position: [1.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0] },
            Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
            Vertex { position: [1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0] },
        ]).unwrap();
    }
}
