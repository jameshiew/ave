use cgmath::{Vector3, Point3};
use std::vec::Vec;
use std;

pub type Position = Point3<f32>;
pub type Direction = Vector3<f32>;

const UP: Direction = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
const DOWN: Direction = Vector3 { x: 0.0, y: -1.0, z: 0.0 };
const NORTH: Direction = Vector3 { x: 0.0, y: 0.0, z: -1.0 };
const SOUTH: Direction = Vector3 { x: 0.0, y: 0.0, z: 1.0 };
const EAST: Direction = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
const WEST: Direction = Vector3 { x: -1.0, y: 0.0, z: 1.0 };

pub trait Adjacent {
    fn adjacent(&self) -> Vec<Self> where Self: std::marker::Sized;
    fn directly_adjacent(&self) -> Vec<Self> where Self: std::marker::Sized;
    fn diagonally_adjacent(&self) -> Vec<Self> where Self: std::marker::Sized;
}
