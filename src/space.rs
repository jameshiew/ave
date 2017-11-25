use cgmath::{Vector3, Point3};
use std::vec::Vec;
use std;

pub type Position = Point3<f32>;
pub type Direction = Vector3<f32>;

pub trait Adjacent {
    fn adjacent(&self) -> Vec<Self> where Self: std::marker::Sized;
    fn directly_adjacent(&self) -> Vec<Self> where Self: std::marker::Sized;
    fn diagonally_adjacent(&self) -> Vec<Self> where Self: std::marker::Sized;
}
