use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3],
}

impl Vertex {
    pub fn new(position: [f32; 3], color: [f32; 3], normal: [f32; 3]) -> Vertex {
        Vertex {
            position,
            color,
            normal,
        }
    }
}

implement_vertex!(Vertex, position, color, normal);

#[allow(dead_code)]
pub enum Shaders {
    None,
    Gouraud,
    Phong,
}

pub fn get_shader<F>(display: &F, shader: Shaders) -> glium::Program
where
    F: glium::backend::Facade,
{
    let program = match shader {
        Shaders::None => glium::Program::from_source(
            display,
            include_str!("./shaders/nolighting.glslv"),
            include_str!("./shaders/nolighting.glslf"),
            None,
        ),
        Shaders::Gouraud => glium::Program::from_source(
            display,
            include_str!("./shaders/gouraud.glslv"),
            include_str!("./shaders/gouraud.glslf"),
            None,
        ),
        Shaders::Phong => glium::Program::from_source(
            display,
            include_str!("./shaders/phong.glslv"),
            include_str!("./shaders/phong.glslf"),
            None,
        ),
    };
    program.unwrap()
}
