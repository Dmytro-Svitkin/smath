#![no_std]

#[inline(always)]
const fn floor(x:f32)->f32{
    let rcl:i32=x as i32;
    if x<rcl as f32{(rcl-1)as f32}else{rcl as f32}
}

#[inline(always)]
const fn round(x:f32)->f32{
  if x>0.0{floor(x+0.5)}
  else {-floor(-x+0.5)}
}

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