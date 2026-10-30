#![no_std]

mod structs;

pub mod generic;
    pub use generic::*;

pub mod constant;
    pub use constant::*;

pub mod trigonometry;
    pub use trigonometry::*;

pub mod vector;
    pub use vector::*;

pub mod matrix;
    pub use matrix::*;