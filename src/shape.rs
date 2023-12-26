use crate::{FuzzyEq, Intersection, Material, Matrix, Ray, Tuple};
use std::any::Any;
use std::fmt;

#[derive(Debug)]
pub struct Props {
    pub material: Material,
    pub transform: Matrix<4>,
}

pub trait Shape {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn shape_eq(&self, other: &dyn Any) -> bool;
    fn normal_at(&self, world_point: Tuple) -> Tuple;
    fn props(&self) -> &Props;
    fn props_mut(&mut self) -> &mut Props;
    fn intersect(&self, ray: Ray) -> Vec<Intersection>;
}

impl Default for Props {
    fn default() -> Self {
        Self {
            transform: Matrix::identity(),
            material: Material::default(),
        }
    }
}

impl fmt::Debug for dyn Shape + '_ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Shape {{ }}",)
    }
}

impl PartialEq for dyn Shape + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.props() == other.props() && self.shape_eq(other.as_any())
    }
}

impl PartialEq for Props {
    fn eq(&self, other: &Self) -> bool {
        self.material == other.material && self.transform.fuzzy_eq(other.transform)
    }
}
