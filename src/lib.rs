#[macro_use]
mod utils;

pub mod canvas;
pub mod matrix;
pub mod tuple;

pub type F = f64;
pub const PI: f64 = std::f64::consts::PI;

const EPSILON: F = 1e-5;
