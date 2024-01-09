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
    fn props(&self) -> &Props;
    fn props_mut(&mut self) -> &mut Props;
    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        // convert into object space
        let ray = ray.transform(self.props().transform.inverse());
        self.local_intersect(ray)
    }
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection>;
    fn normal_at(&self, point: Tuple) -> Tuple {
        let local_point = self.props().transform.inverse() * point;
        let local_normal = self.local_normal_at(local_point);

        let mut world_normal = self.props().transform.inverse().transpose() * local_normal;
        world_normal.w = 0.0;

        world_normal.normalize()
    }
    fn local_normal_at(&self, point: Tuple) -> Tuple;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use test_shape::TestShape;

    static mut SAVED_RAY: Option<Ray> = None;

    mod test_shape {
        use super::*;

        #[derive(Debug, Default)]
        pub struct TestShape {
            props: Props,
        }

        impl TestShape {
            pub fn new() -> Self {
                Self {
                    props: Props::default(),
                }
            }
        }

        impl Shape for TestShape {
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }

            fn shape_eq(&self, other: &dyn Any) -> bool {
                other.downcast_ref::<Self>().is_some()
            }

            fn props(&self) -> &Props {
                &self.props
            }

            fn props_mut(&mut self) -> &mut Props {
                &mut self.props
            }

            fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
                unsafe { SAVED_RAY = Some(ray) };

                vec![]
            }

            fn local_normal_at(&self, point: Tuple) -> Tuple {
                v(point.x, point.y, point.z)
            }
        }
    }

    #[test]
    fn the_default_transformation() {
        let s = TestShape::new();

        assert_fuzzy_eq!(s.props().transform, Matrix::identity())
    }

    #[test]
    fn assigning_a_transformation() {
        let mut s = TestShape::new();
        s.props_mut().transform = Matrix::translation(2, 3, 4);

        assert_fuzzy_eq!(s.props().transform, Matrix::translation(2, 3, 4))
    }

    #[test]
    fn the_default_material() {
        let s = TestShape::new();
        let m = s.props().material;

        assert_eq!(m, Material::new())
    }

    #[test]
    fn assigning_a_material() {
        let mut s = TestShape::new();
        let m = Material::new().ambient(1);
        s.props_mut().material = m;

        assert_eq!(s.props().material, m)
    }

    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let r = ray(pt(0, 0, -5), v(0, 0, 1));
        let mut s = TestShape::new();
        s.props_mut().transform = Matrix::scaling(2, 2, 2);

        unsafe {
            SAVED_RAY = None;
            s.intersect(r);

            assert_eq!(SAVED_RAY.unwrap().origin, pt(0, 0, -2.5));
            assert_eq!(SAVED_RAY.unwrap().direction, v(0, 0, 0.5));
        }
    }

    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let r = ray(pt(0, 0, -5), v(0, 0, 1));
        let mut s = TestShape::new();
        s.props_mut().transform = Matrix::translation(5, 0, 0);

        unsafe {
            SAVED_RAY = None;
            s.intersect(r);

            assert_eq!(SAVED_RAY.unwrap().origin, pt(-5, 0, -5));
            assert_eq!(SAVED_RAY.unwrap().direction, v(0, 0, 1));
        }
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let mut s = TestShape::new();
        let m = Matrix::scaling(1, 0.5, 1) * Matrix::rotation_z(PI / 5.0);
        s.props_mut().transform = m;
        let n = s.normal_at(pt(0, F::sqrt(2.0) / 2.0, -F::sqrt(2.0) / 2.0));

        assert_fuzzy_eq!(n, v(0, 0.97014, -0.24254));
    }
}
