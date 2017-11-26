use render::Vertex;
use std::hash::{Hash, Hasher};
use glium::vertex::VertexBuffer;
use glium::backend::Facade;
use color::Color;
use color;
use space;
use space::{Position, Direction};

/// A block type's ID is its index into the global BLOCKS array
pub type ID = usize;

/// Size of a block (in metres)
const BLOCK_SIZE: f32 = 1.0;

/// Vertices of a cube
///
/// ordering is important - so that the correct faces get culled
const CUBE_VERTICES: [Position; 24] = [
    // face
    Position { x: 0.0,        y: 0.0,        z: BLOCK_SIZE },
    Position { x: BLOCK_SIZE, y: 0.0,        z: BLOCK_SIZE },
    Position { x: 0.0,        y: BLOCK_SIZE, z: BLOCK_SIZE },
    Position { x: BLOCK_SIZE, y: BLOCK_SIZE, z: BLOCK_SIZE },
    // face
    Position { x: BLOCK_SIZE, y: 0.0,        z: BLOCK_SIZE },
    Position { x: BLOCK_SIZE, y: 0.0,        z: 0.0        },
    Position { x: BLOCK_SIZE, y: BLOCK_SIZE, z: BLOCK_SIZE },
    Position { x: BLOCK_SIZE, y: BLOCK_SIZE, z: 0.0        },
    // face
    Position { x: BLOCK_SIZE, y: 0.0,        z: 0.0        },
    Position { x: 0.0,        y: 0.0,        z: 0.0        },
    Position { x: BLOCK_SIZE, y: BLOCK_SIZE, z: 0.0        },
    Position { x: 0.0,        y: BLOCK_SIZE, z: 0.0        },
    // face
    Position { x: 0.0,        y: 0.0,        z: 0.0        },
    Position { x: 0.0,        y: 0.0,        z: BLOCK_SIZE },
    Position { x: 0.0,        y: BLOCK_SIZE, z: 0.0        },
    Position { x: 0.0,        y: BLOCK_SIZE, z: BLOCK_SIZE },
    // face
    Position { x: 0.0,        y: 0.0,        z: 0.0        },
    Position { x: BLOCK_SIZE, y: 0.0,        z: 0.0        },
    Position { x: 0.0,        y: 0.0,        z: BLOCK_SIZE },
    Position { x: BLOCK_SIZE, y: 0.0,        z: BLOCK_SIZE },
    // face
    Position { x: 0.0,        y: BLOCK_SIZE, z: BLOCK_SIZE },
    Position { x: BLOCK_SIZE, y: BLOCK_SIZE, z: BLOCK_SIZE },
    Position { x: 0.0,        y: BLOCK_SIZE, z: 0.0        },
    Position { x: BLOCK_SIZE, y: BLOCK_SIZE, z: 0.0        },
];

const CUBE_NORMALS: [Direction; 6] = [
    space::SOUTH,
    space::EAST,
    space::NORTH,
    space::WEST,
    space::DOWN,
    space::UP,
];

pub fn cube_at(position: &Position) -> [Position; 8] {
    let x = position.x;
    let y = position.y;
    let z = position.z;
    return [
        [x,       y,       z      ].into(),
        [x,       y,       z + 1.0].into(),
        [x,       y + 1.0, z      ].into(),
        [x,       y + 1.0, z + 1.0].into(),
        [x + 1.0, y,       z      ].into(),
        [x + 1.0, y,       z + 1.0].into(),
        [x + 1.0, y + 1.0, z      ].into(),
        [x + 1.0, y + 1.0, z + 1.0].into(),
    ]
}

/// Create a vertex buffer for a cube centred at (x, y, z)
pub fn make_cube<F: ? Sized>(facade: &F, position: &Position, color: Color) -> VertexBuffer<Vertex> where F: Facade {
    let x = position[0];
    let y = position[1];
    let z = position[2];
    VertexBuffer::new(facade, &[
        Vertex::new([x + CUBE_VERTICES[0][0], y + CUBE_VERTICES[0][1], z + CUBE_VERTICES[0][2]], color, CUBE_NORMALS[0].into()),
        Vertex::new([x + CUBE_VERTICES[1][0], y + CUBE_VERTICES[1][1], z + CUBE_VERTICES[1][2]], color, CUBE_NORMALS[0].into()),
        Vertex::new([x + CUBE_VERTICES[2][0], y + CUBE_VERTICES[2][1], z + CUBE_VERTICES[2][2]], color, CUBE_NORMALS[0].into()),
        Vertex::new([x + CUBE_VERTICES[3][0], y + CUBE_VERTICES[3][1], z + CUBE_VERTICES[3][2]], color, CUBE_NORMALS[0].into()),

        Vertex::new([x + CUBE_VERTICES[4][0], y + CUBE_VERTICES[4][1], z + CUBE_VERTICES[4][2]], color, CUBE_NORMALS[1].into()),
        Vertex::new([x + CUBE_VERTICES[5][0], y + CUBE_VERTICES[5][1], z + CUBE_VERTICES[5][2]], color, CUBE_NORMALS[1].into()),
        Vertex::new([x + CUBE_VERTICES[6][0], y + CUBE_VERTICES[6][1], z + CUBE_VERTICES[6][2]], color, CUBE_NORMALS[1].into()),
        Vertex::new([x + CUBE_VERTICES[7][0], y + CUBE_VERTICES[7][1], z + CUBE_VERTICES[7][2]], color, CUBE_NORMALS[1].into()),

        Vertex::new([x + CUBE_VERTICES[8][0], y + CUBE_VERTICES[8][1], z + CUBE_VERTICES[8][2]], color, CUBE_NORMALS[2].into()),
        Vertex::new([x + CUBE_VERTICES[9][0], y + CUBE_VERTICES[9][1], z + CUBE_VERTICES[9][2]], color, CUBE_NORMALS[2].into()),
        Vertex::new([x + CUBE_VERTICES[10][0], y + CUBE_VERTICES[10][1], z + CUBE_VERTICES[10][2]], color, CUBE_NORMALS[2].into()),
        Vertex::new([x + CUBE_VERTICES[11][0], y + CUBE_VERTICES[11][1], z + CUBE_VERTICES[11][2]], color, CUBE_NORMALS[2].into()),

        Vertex::new([x + CUBE_VERTICES[12][0], y + CUBE_VERTICES[12][1], z + CUBE_VERTICES[12][2]], color, CUBE_NORMALS[3].into()),
        Vertex::new([x + CUBE_VERTICES[13][0], y + CUBE_VERTICES[13][1], z + CUBE_VERTICES[13][2]], color, CUBE_NORMALS[3].into()),
        Vertex::new([x + CUBE_VERTICES[14][0], y + CUBE_VERTICES[14][1], z + CUBE_VERTICES[14][2]], color, CUBE_NORMALS[3].into()),
        Vertex::new([x + CUBE_VERTICES[15][0], y + CUBE_VERTICES[15][1], z + CUBE_VERTICES[15][2]], color, CUBE_NORMALS[3].into()),

        Vertex::new([x + CUBE_VERTICES[16][0], y + CUBE_VERTICES[16][1], z + CUBE_VERTICES[16][2]], color, CUBE_NORMALS[4].into()),
        Vertex::new([x + CUBE_VERTICES[17][0], y + CUBE_VERTICES[17][1], z + CUBE_VERTICES[17][2]], color, CUBE_NORMALS[4].into()),
        Vertex::new([x + CUBE_VERTICES[18][0], y + CUBE_VERTICES[18][1], z + CUBE_VERTICES[18][2]], color, CUBE_NORMALS[4].into()),
        Vertex::new([x + CUBE_VERTICES[19][0], y + CUBE_VERTICES[19][1], z + CUBE_VERTICES[19][2]], color, CUBE_NORMALS[4].into()),

        Vertex::new([x + CUBE_VERTICES[20][0], y + CUBE_VERTICES[20][1], z + CUBE_VERTICES[20][2]], color, CUBE_NORMALS[5].into()),
        Vertex::new([x + CUBE_VERTICES[21][0], y + CUBE_VERTICES[21][1], z + CUBE_VERTICES[21][2]], color, CUBE_NORMALS[5].into()),
        Vertex::new([x + CUBE_VERTICES[22][0], y + CUBE_VERTICES[22][1], z + CUBE_VERTICES[22][2]], color, CUBE_NORMALS[5].into()),
        Vertex::new([x + CUBE_VERTICES[23][0], y + CUBE_VERTICES[23][1], z + CUBE_VERTICES[23][2]], color, CUBE_NORMALS[5].into()),
    ]).unwrap()
}

#[derive(Debug)]
pub struct BlockType {
    pub name: &'static str,
    pub color: Color,
}

impl Hash for BlockType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl PartialEq for BlockType {
    fn eq(&self, other: &BlockType) -> bool {
        self.name == other.name
    }
}

impl Eq for BlockType {}

pub static BLOCKS: [&BlockType; 2] = [
    &BlockType {
        name: "grass",
        color: color::GREEN,
    },
    &BlockType {
        name: "dirt",
        color: color::BROWN,
    },
];
