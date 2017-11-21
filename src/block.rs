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
    pub fn new(x: f32, y: f32, z: f32) -> Block {
        Block { x, y, z }
    }

    pub fn get_vertices<F: ? Sized>(self, facade: &F) -> VertexBuffer<Vertex> where F: Facade {
        return VertexBuffer::new(facade, &[
            Vertex { position: [self.x - 1.0, self.y - 1.0, self.z - 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [0.0, 1.0]},
            Vertex { position: [self.x - 1.0, self.y - 1.0, self.z + 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [1.0, 1.0]},
            Vertex { position: [self.x - 1.0, self.y + 1.0, self.z - 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [0.0, 0.0]},
            Vertex { position: [self.x - 1.0, self.y + 1.0, self.z + 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [1.0, 0.0]},

            Vertex { position: [self.x + 1.0, self.y - 1.0, self.z - 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [0.0, 1.0]},
            Vertex { position: [self.x + 1.0, self.y - 1.0, self.z + 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [1.0, 1.0]},
            Vertex { position: [self.x + 1.0, self.y + 1.0, self.z - 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [0.0, 0.0]},
            Vertex { position: [self.x + 1.0, self.y + 1.0, self.z + 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [1.0, 0.0]},

            Vertex { position: [self.x - 1.0, self.y + 1.0, self.z - 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [0.0, 1.0]},
            Vertex { position: [self.x - 1.0, self.y + 1.0, self.z + 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [1.0, 1.0]},
            Vertex { position: [self.x + 1.0, self.y + 1.0, self.z - 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [0.0, 0.0]},
            Vertex { position: [self.x + 1.0, self.y + 1.0, self.z + 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [1.0, 0.0]},

            Vertex { position: [self.x - 1.0, self.y - 1.0, self.z - 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [0.0, 1.0]},
            Vertex { position: [self.x - 1.0, self.y - 1.0, self.z + 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [1.0, 1.0]},
            Vertex { position: [self.x + 1.0, self.y - 1.0, self.z - 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [0.0, 0.0]},
            Vertex { position: [self.x + 1.0, self.y - 1.0, self.z + 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [1.0, 0.0]},

            Vertex { position: [self.x - 1.0, self.y - 1.0, self.z - 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [0.0, 1.0]},
            Vertex { position: [self.x - 1.0, self.y + 1.0, self.z - 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [1.0, 1.0]},
            Vertex { position: [self.x + 1.0, self.y - 1.0, self.z - 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [0.0, 0.0]},
            Vertex { position: [self.x + 1.0, self.y + 1.0, self.z - 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [1.0, 0.0]},

            Vertex { position: [self.x - 1.0, self.y - 1.0, self.z + 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [0.0, 1.0]},
            Vertex { position: [self.x - 1.0, self.y + 1.0, self.z + 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [1.0, 1.0]},
            Vertex { position: [self.x + 1.0, self.y - 1.0, self.z + 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [0.0, 0.0]},
            Vertex { position: [self.x + 1.0, self.y + 1.0, self.z + 1.0], normal: [0.0, 0.0, self.z - 1.0], tex_coords: [1.0, 0.0]}
        ]).unwrap();
    }
}
