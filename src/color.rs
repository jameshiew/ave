use rand;
use rand::Rng;

pub type Color = [f32; 3];

pub fn get_random_color() -> Color {
    let mut rng = rand::thread_rng();
    [rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)]
}

pub const BROWN: Color = [0.545, 0.271, 0.075];
pub const GREEN: Color = [0.196, 0.804, 0.196];
pub const SKY: Color = [0.529, 0.808, 0.980];
