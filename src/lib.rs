#[macro_use]
mod utils;
#[macro_use]
mod matrix;

mod camera;
mod canvas;
mod intersection;
mod material;
mod pattern;
mod plane;
mod ray;
mod shape;
mod sphere;
mod transformation;
mod tuple;
mod world;

pub use camera::Camera;
pub use canvas::{color, Canvas, Color};
pub use intersection::{Comps, Intersection, Intersections};
pub use material::Material;
pub use matrix::Matrix;
pub use pattern::{checkers_pattern, gradient_pattern, ring_pattern, stripe_pattern, Pattern};
pub use plane::Plane;
pub use ray::{point_light, ray, PointLight, Ray};
pub use shape::{Props, Shape};
pub use sphere::Sphere;
pub use transformation::view_transform;
pub use tuple::{point as pt, vector as v, Tuple};
pub use utils::FuzzyEq;
pub use world::World;

pub type F = f64;
pub const PI: F = std::f64::consts::PI;

const EPSILON: F = 1e-5;
