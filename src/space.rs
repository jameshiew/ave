use cgmath::{Vector3, Point3};
use collision::Plane;
use std::vec::Vec;
use std;

pub const MODEL: [[f32; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

pub type Position = Point3<f32>;
pub type Direction = Vector3<f32>;

pub const UP: Direction = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
pub const DOWN: Direction = Vector3 { x: 0.0, y: -1.0, z: 0.0 };
pub const NORTH: Direction = Vector3 { x: 0.0, y: 0.0, z: -1.0 };
pub const SOUTH: Direction = Vector3 { x: 0.0, y: 0.0, z: 1.0 };
pub const EAST: Direction = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
pub const WEST: Direction = Vector3 { x: -1.0, y: 0.0, z: 1.0 };

pub trait Adjacent {
    fn adjacent(&self) -> Vec<Self> where Self: std::marker::Sized;
    fn directly_adjacent(&self) -> Vec<Self> where Self: std::marker::Sized;
    fn diagonally_adjacent(&self) -> Vec<Self> where Self: std::marker::Sized;
}


/// See https://www.khronos.org/opengl/wiki/Calculating_a_Surface_Normal
pub fn get_normal_for_triangle(point_a: Position, point_b: Position, point_c: Position) -> Direction {
    Plane::from_points(point_a, point_b, point_c).unwrap().n
}


#[cfg(test)]
mod tests {
    use space::get_normal_for_triangle;

//    #[test]
//    fn test_get_normal_for_triangle() {
//        assert_ne!(get_normal_for_triangle(
//            [0.0, 0.0, 0.0].into(),
//            [1.0, 0.0, 0.0].into(),
//            [0.0, 3.0, 0.0].into(),
//        ),
//    }
}
