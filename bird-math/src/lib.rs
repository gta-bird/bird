pub use std::{ffi::c_void, mem::MaybeUninit};
pub use cgmath::*;

pub mod vector;

pub trait FromGame {
    fn from_game(data: MaybeUninit<*mut c_void>) -> Self;
}
