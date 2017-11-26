#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3],
}

impl Vertex {
    pub fn new(position: [f32; 3], color: [f32; 3], normal: [f32; 3]) -> Vertex {
        Vertex {position, color, normal}
    }
}

implement_vertex!(Vertex, position, color, normal);
