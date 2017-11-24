use rand;
use rand::Rng;

pub type Color = [f32; 3];

pub fn get_random_color() -> Color {
    let mut rng = rand::thread_rng();
    [rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)]
}
