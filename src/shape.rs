use crate::{Material, Tuple};
use std::any::Any;
use std::fmt;

pub trait Shape {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn shape_eq(&self, other: &dyn Any) -> bool;
    fn material(&self) -> &Material;
    fn normal_at(&self, world_point: Tuple) -> Tuple;
}

impl fmt::Debug for dyn Shape + '_ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Shape {{ }}",)
    }
}

impl PartialEq for dyn Shape + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.shape_eq(other.as_any())
    }
}
