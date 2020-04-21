use render::Vertex;
use std::hash::{Hash, Hasher};
use glium::vertex::VertexBuffer;
use glium::backend::Facade;
use color::Color;
use space;
use space::{Position, Direction};

/// Size of a block (in metres)
const BLOCK_SIZE: f32 = 1.0;

/// Vertices of a cube
///
/// ordering is important - so that the correct faces get culled
const CUBE_VERTICES: [Position; 24] = [
    // south
    Position { x: 0.0,        y: 0.0,        z: BLOCK_SIZE },
    Position { x: BLOCK_SIZE, y: 0.0,        z: BLOCK_SIZE },
    Position { x: 0.0,        y: BLOCK_SIZE, z: BLOCK_SIZE },
    Position { x: BLOCK_SIZE, y: BLOCK_SIZE, z: BLOCK_SIZE },
    // east
    Position { x: BLOCK_SIZE, y: 0.0,        z: BLOCK_SIZE },
    Position { x: BLOCK_SIZE, y: 0.0,        z: 0.0        },
    Position { x: BLOCK_SIZE, y: BLOCK_SIZE, z: BLOCK_SIZE },
    Position { x: BLOCK_SIZE, y: BLOCK_SIZE, z: 0.0        },
    // north
    Position { x: BLOCK_SIZE, y: 0.0,        z: 0.0        },
    Position { x: 0.0,        y: 0.0,        z: 0.0        },
    Position { x: BLOCK_SIZE, y: BLOCK_SIZE, z: 0.0        },
    Position { x: 0.0,        y: BLOCK_SIZE, z: 0.0        },
    // west
    Position { x: 0.0,        y: 0.0,        z: 0.0        },
    Position { x: 0.0,        y: 0.0,        z: BLOCK_SIZE },
    Position { x: 0.0,        y: BLOCK_SIZE, z: 0.0        },
    Position { x: 0.0,        y: BLOCK_SIZE, z: BLOCK_SIZE },
    // down
    Position { x: 0.0,        y: 0.0,        z: 0.0        },
    Position { x: BLOCK_SIZE, y: 0.0,        z: 0.0        },
    Position { x: 0.0,        y: 0.0,        z: BLOCK_SIZE },
    Position { x: BLOCK_SIZE, y: 0.0,        z: BLOCK_SIZE },
    // up
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
    [
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

pub struct Mask {
    up: bool,
    down: bool,
    north: bool,
    south: bool,
    east: bool,
    west: bool,
}

impl Mask {
    pub fn new() -> Mask {
        Mask {
            up: false,
            down: false,
            north: false,
            south: false,
            east: false,
            west: false,
        }
    }
}

/// Create a vertex buffer for a cube centred at (x, y, z)
pub fn make_cube<F: ? Sized>(facade: &F, position: &Position, color: Color, mask: Mask) -> VertexBuffer<Vertex> where F: Facade {
    let x = position[0];
    let y = position[1];
    let z = position[2];
    let mut vertices = Vec::new();
    if !mask.south {
        vertices.push(Vertex::new([x + CUBE_VERTICES[0][0], y + CUBE_VERTICES[0][1], z + CUBE_VERTICES[0][2]], color, CUBE_NORMALS[0].into()));
        vertices.push(Vertex::new([x + CUBE_VERTICES[1][0], y + CUBE_VERTICES[1][1], z + CUBE_VERTICES[1][2]], color, CUBE_NORMALS[0].into()));
        vertices.push(Vertex::new([x + CUBE_VERTICES[2][0], y + CUBE_VERTICES[2][1], z + CUBE_VERTICES[2][2]], color, CUBE_NORMALS[0].into()));
        vertices.push(Vertex::new([x + CUBE_VERTICES[3][0], y + CUBE_VERTICES[3][1], z + CUBE_VERTICES[3][2]], color, CUBE_NORMALS[0].into()));
    }
    if !mask.east {
        vertices.push(Vertex::new([x + CUBE_VERTICES[4][0], y + CUBE_VERTICES[4][1], z + CUBE_VERTICES[4][2]], color, CUBE_NORMALS[1].into()));
        vertices.push(Vertex::new([x + CUBE_VERTICES[5][0], y + CUBE_VERTICES[5][1], z + CUBE_VERTICES[5][2]], color, CUBE_NORMALS[1].into()));
        vertices.push(Vertex::new([x + CUBE_VERTICES[6][0], y + CUBE_VERTICES[6][1], z + CUBE_VERTICES[6][2]], color, CUBE_NORMALS[1].into()));
        vertices.push(Vertex::new([x + CUBE_VERTICES[7][0], y + CUBE_VERTICES[7][1], z + CUBE_VERTICES[7][2]], color, CUBE_NORMALS[1].into()));
    }
    if !mask.north {
        vertices.push(Vertex::new([x + CUBE_VERTICES[8][0], y + CUBE_VERTICES[8][1], z + CUBE_VERTICES[8][2]], color, CUBE_NORMALS[2].into()));
        vertices.push(Vertex::new([x + CUBE_VERTICES[9][0], y + CUBE_VERTICES[9][1], z + CUBE_VERTICES[9][2]], color, CUBE_NORMALS[2].into()));
        vertices.push(Vertex::new([x + CUBE_VERTICES[10][0], y + CUBE_VERTICES[10][1], z + CUBE_VERTICES[10][2]], color, CUBE_NORMALS[2].into()));
        vertices.push(Vertex::new([x + CUBE_VERTICES[11][0], y + CUBE_VERTICES[11][1], z + CUBE_VERTICES[11][2]], color, CUBE_NORMALS[2].into()));
    }
    if !mask.west {
        vertices.push(Vertex::new([x + CUBE_VERTICES[12][0], y + CUBE_VERTICES[12][1], z + CUBE_VERTICES[12][2]], color, CUBE_NORMALS[3].into()));
        vertices.push(Vertex::new([x + CUBE_VERTICES[13][0], y + CUBE_VERTICES[13][1], z + CUBE_VERTICES[13][2]], color, CUBE_NORMALS[3].into()));
        vertices.push(Vertex::new([x + CUBE_VERTICES[14][0], y + CUBE_VERTICES[14][1], z + CUBE_VERTICES[14][2]], color, CUBE_NORMALS[3].into()));
        vertices.push(Vertex::new([x + CUBE_VERTICES[15][0], y + CUBE_VERTICES[15][1], z + CUBE_VERTICES[15][2]], color, CUBE_NORMALS[3].into()));
    }
    if !mask.down {
        vertices.push(Vertex::new([x + CUBE_VERTICES[16][0], y + CUBE_VERTICES[16][1], z + CUBE_VERTICES[16][2]], color, CUBE_NORMALS[4].into()));
        vertices.push(Vertex::new([x + CUBE_VERTICES[17][0], y + CUBE_VERTICES[17][1], z + CUBE_VERTICES[17][2]], color, CUBE_NORMALS[4].into()));
        vertices.push(Vertex::new([x + CUBE_VERTICES[18][0], y + CUBE_VERTICES[18][1], z + CUBE_VERTICES[18][2]], color, CUBE_NORMALS[4].into()));
        vertices.push(Vertex::new([x + CUBE_VERTICES[19][0], y + CUBE_VERTICES[19][1], z + CUBE_VERTICES[19][2]], color, CUBE_NORMALS[4].into()));
    }
    if !mask.up {
        vertices.push(Vertex::new([x + CUBE_VERTICES[20][0], y + CUBE_VERTICES[20][1], z + CUBE_VERTICES[20][2]], color, CUBE_NORMALS[5].into()));
        vertices.push(Vertex::new([x + CUBE_VERTICES[21][0], y + CUBE_VERTICES[21][1], z + CUBE_VERTICES[21][2]], color, CUBE_NORMALS[5].into()));
        vertices.push(Vertex::new([x + CUBE_VERTICES[22][0], y + CUBE_VERTICES[22][1], z + CUBE_VERTICES[22][2]], color, CUBE_NORMALS[5].into()));
        vertices.push(Vertex::new([x + CUBE_VERTICES[23][0], y + CUBE_VERTICES[23][1], z + CUBE_VERTICES[23][2]], color, CUBE_NORMALS[5].into()));
    }
    VertexBuffer::new(facade, vertices.as_slice()).unwrap()
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

pub static GRASS: &BlockType = &BlockType {
    name: "grass",
    color: [0.196, 0.804, 0.196],
};

pub static DIRT: &BlockType = &BlockType {
    name: "dirt",
    color: [0.545, 0.271, 0.075],
};

pub static STONE: &BlockType = &BlockType {
    name: "stone",
    color: [0.827, 0.827, 0.827],
};

#[allow(dead_code)]  // use as and when
pub static WATER: &BlockType = &BlockType {
    name: "water",
    color: [0.498, 1.000, 0.831],
};

pub static SAND: &BlockType = &BlockType {
    name: "sand",
    color: [0.941, 0.902, 0.549],
};
