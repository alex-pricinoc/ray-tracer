#[macro_use]
mod utils;

mod canvas;
mod intersection;
mod matrix;
mod ray;
mod shape;
mod sphere;
mod tuple;

pub use canvas::{color, Canvas};
pub use intersection::{Intersection, Intersections};
pub use matrix::Matrix;
pub use ray::Ray;
pub use shape::Shape;
pub use sphere::Sphere;
pub use tuple::{point, vector, Tuple};
pub use utils::FuzzyEq;

pub type F = f64;
pub const PI: F = std::f64::consts::PI;

const EPSILON: F = 1e-5;
