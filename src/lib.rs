#![no_std]

pub mod generic;
    pub use generic::*;

pub mod constant;
    pub use constant::*;

pub mod trigonometry;
    pub use trigonometry::*;

/*
pub mod conversion;
    pub use conversion::*;
    pub use conversion::length::*;
    pub use conversion::weight::*;
    pub use conversion::volume::*;
    pub use conversion::speed::*;
    pub use conversion::pressure::*;
    pub use conversion::energy::*;
    pub use conversion::time::*;
    pub use conversion::temperature::*;
*/

pub mod vector;
    pub use vector::*;

pub mod matrix;
    pub use matrix::*;