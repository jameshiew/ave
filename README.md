# ave - Another Voxel Engine [![Build Status](https://travis-ci.org/jameshiew/ave.svg?branch=master)](https://travis-ci.org/jameshiew/ave)

![Screenshot](screenshot.png "Screenshot")

Playing around with voxel-based procedural generation in Rust.

Most of the code is not very efficient, the rendering code in particular is pretty forced - and there is no multithreading, so lots of room for improvement. Built with lots of help from the [Glium](https://github.com/glium/glium) and [Glutin](https://github.com/tomaka/glutin) examples - most of the boilerplate code and shaders are adapted from there.

## Getting Started

Rust and Cargo need to be installed. Tested against Rust 1.22.1

1. Run `cargo run --release` from a shell
2. WASD to move; arrow keys to rotate the camera; Q and E to speed up and slow down, respectively

Some default values such as render distance can be edited in `src/default.rs`
