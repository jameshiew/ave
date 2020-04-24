use cgmath::{Point3, Vector3};
use std::vec::Vec;

pub const MODEL: [[f32; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

pub type Position = Point3<f32>;
pub type Direction = Vector3<f32>;

pub const UP: Direction = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};
pub const DOWN: Direction = Vector3 {
    x: 0.0,
    y: -1.0,
    z: 0.0,
};
pub const NORTH: Direction = Vector3 {
    x: 0.0,
    y: 0.0,
    z: -1.0,
};
pub const SOUTH: Direction = Vector3 {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};
pub const EAST: Direction = Vector3 {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};
pub const WEST: Direction = Vector3 {
    x: -1.0,
    y: 0.0,
    z: 0.0,
};

pub trait Adjacent {
    fn adjacent(&self) -> Vec<Self>
    where
        Self: std::marker::Sized;
    fn directly_adjacent(&self) -> Vec<Self>
    where
        Self: std::marker::Sized;
    fn diagonally_adjacent(&self) -> Vec<Self>
    where
        Self: std::marker::Sized;
}
