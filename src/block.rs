use vertex::Vertex;
use std::hash::{Hash, Hasher};
use glium::vertex::VertexBuffer;
use glium::backend::Facade;
use color::Color;
use color;

/// Size of a block (in metres)
const BLOCK_SIZE: f32 = 0.5;

/// Vertices of a cube
///
/// ordering is important - so that the correct faces get culled
static CUBE: [[f32; 3]; 24] = [
    // face
    [-BLOCK_SIZE, -BLOCK_SIZE, BLOCK_SIZE],
    [BLOCK_SIZE, -BLOCK_SIZE, BLOCK_SIZE],
    [-BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE],
    [BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE],
    // face
    [BLOCK_SIZE, -BLOCK_SIZE, BLOCK_SIZE],
    [BLOCK_SIZE, -BLOCK_SIZE, -BLOCK_SIZE],
    [BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE],
    [BLOCK_SIZE, BLOCK_SIZE, -BLOCK_SIZE],
    // face
    [BLOCK_SIZE, -BLOCK_SIZE, -BLOCK_SIZE],
    [-BLOCK_SIZE, -BLOCK_SIZE, -BLOCK_SIZE],
    [BLOCK_SIZE, BLOCK_SIZE, -BLOCK_SIZE],
    [-BLOCK_SIZE, BLOCK_SIZE, -BLOCK_SIZE],
    // face
    [-BLOCK_SIZE, -BLOCK_SIZE, -BLOCK_SIZE],
    [-BLOCK_SIZE, -BLOCK_SIZE, BLOCK_SIZE],
    [-BLOCK_SIZE, BLOCK_SIZE, -BLOCK_SIZE],
    [-BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE],
    // face
    [-BLOCK_SIZE, -BLOCK_SIZE, -BLOCK_SIZE],
    [BLOCK_SIZE, -BLOCK_SIZE, -BLOCK_SIZE],
    [-BLOCK_SIZE, -BLOCK_SIZE, BLOCK_SIZE],
    [BLOCK_SIZE, -BLOCK_SIZE, BLOCK_SIZE],
    // face
    [-BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE],
    [BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE],
    [-BLOCK_SIZE, BLOCK_SIZE, -BLOCK_SIZE],
    [BLOCK_SIZE, BLOCK_SIZE, -BLOCK_SIZE],
];

/// Create a vertex buffer for a cube centred at (x, y, z)
pub fn make_cube<F: ? Sized>(facade: &F, x: f32, y: f32, z: f32, color: Color) -> VertexBuffer<Vertex> where F: Facade {
    VertexBuffer::new(facade, &[
        Vertex::new([x + CUBE[0][0], y + CUBE[0][1], z + CUBE[0][2]], color),
        Vertex::new([x + CUBE[1][0], y + CUBE[1][1], z + CUBE[1][2]], color),
        Vertex::new([x + CUBE[2][0], y + CUBE[2][1], z + CUBE[2][2]], color),
        Vertex::new([x + CUBE[3][0], y + CUBE[3][1], z + CUBE[3][2]], color),
        Vertex::new([x + CUBE[4][0], y + CUBE[4][1], z + CUBE[4][2]], color),
        Vertex::new([x + CUBE[5][0], y + CUBE[5][1], z + CUBE[5][2]], color),
        Vertex::new([x + CUBE[6][0], y + CUBE[6][1], z + CUBE[6][2]], color),
        Vertex::new([x + CUBE[7][0], y + CUBE[7][1], z + CUBE[7][2]], color),
        Vertex::new([x + CUBE[8][0], y + CUBE[8][1], z + CUBE[8][2]], color),
        Vertex::new([x + CUBE[9][0], y + CUBE[9][1], z + CUBE[9][2]], color),
        Vertex::new([x + CUBE[10][0], y + CUBE[10][1], z + CUBE[10][2]], color),
        Vertex::new([x + CUBE[11][0], y + CUBE[11][1], z + CUBE[11][2]], color),
        Vertex::new([x + CUBE[12][0], y + CUBE[12][1], z + CUBE[12][2]], color),
        Vertex::new([x + CUBE[13][0], y + CUBE[13][1], z + CUBE[13][2]], color),
        Vertex::new([x + CUBE[14][0], y + CUBE[14][1], z + CUBE[14][2]], color),
        Vertex::new([x + CUBE[15][0], y + CUBE[15][1], z + CUBE[15][2]], color),
        Vertex::new([x + CUBE[16][0], y + CUBE[16][1], z + CUBE[16][2]], color),
        Vertex::new([x + CUBE[17][0], y + CUBE[17][1], z + CUBE[17][2]], color),
        Vertex::new([x + CUBE[18][0], y + CUBE[18][1], z + CUBE[18][2]], color),
        Vertex::new([x + CUBE[19][0], y + CUBE[19][1], z + CUBE[19][2]], color),
        Vertex::new([x + CUBE[20][0], y + CUBE[20][1], z + CUBE[20][2]], color),
        Vertex::new([x + CUBE[21][0], y + CUBE[21][1], z + CUBE[21][2]], color),
        Vertex::new([x + CUBE[22][0], y + CUBE[22][1], z + CUBE[22][2]], color),
        Vertex::new([x + CUBE[23][0], y + CUBE[23][1], z + CUBE[23][2]], color),
    ]).unwrap()
}

#[derive(Debug)]
pub struct BlockType {
    pub id: u8,
    pub color: Color,
}

impl Hash for BlockType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialEq for BlockType {
    fn eq(&self, other: &BlockType) -> bool {
        self.id == other.id
    }
}

impl Eq for BlockType {}

pub static BLOCKS: [&BlockType; 2] = [
    &BlockType {
        id: 0,
        color: color::GREEN,
    },
    &BlockType {
        id: 1,
        color: color::BROWN,
    },
];
