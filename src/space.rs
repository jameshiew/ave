extern crate float_cmp;

use self::float_cmp::ApproxEqUlps;

#[derive(Debug)]
pub struct Position(pub f32, pub f32, pub f32);

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.0.approx_eq_ulps(&other.0, 2) && self.1.approx_eq_ulps(&other.1, 2) && self.2.approx_eq_ulps(&other.2, 2)
    }
}
impl Eq for Position {}

#[derive(Debug)]
pub struct Direction(pub f32, pub f32, pub f32);

impl PartialEq for Direction {
    fn eq(&self, other: &Direction) -> bool {
        self.0.approx_eq_ulps(&other.0, 2) && self.1.approx_eq_ulps(&other.1, 2) && self.2.approx_eq_ulps(&other.2, 2)
    }
}
impl Eq for Direction {}
