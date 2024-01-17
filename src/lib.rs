#[macro_use]
mod utils;
#[macro_use]
mod matrix;

mod camera;
mod canvas;
mod intersection;
mod material;
mod pattern;
mod ray;
mod shapes;
mod transformation;
mod tuple;
mod world;

pub use camera::Camera;
pub use canvas::{color, Canvas, Color, BLACK, WHITE};
pub use intersection::{Comps, Intersection, Intersections};
pub use material::Material;
pub use matrix::Matrix;
pub use pattern::{checkers, gradient, ring, stripe, Pattern};
pub use ray::{point_light, ray, PointLight, Ray};
pub use shapes::cube::Cube;
pub use shapes::plane::{glass as glass_plane, Plane};
pub use shapes::sphere::{glass as glass_sphere, Sphere};
pub use shapes::{Props, Shape};
pub use transformation::view_transform;
pub use tuple::{point as pt, vector as v, Tuple};
pub use utils::FuzzyEq;
pub use world::World;

pub type F = f64;
pub const PI: F = std::f64::consts::PI;

const EPSILON: F = 1e-5;
const REFLECTION_DEPTH: u8 = 5;
