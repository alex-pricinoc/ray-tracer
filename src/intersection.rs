use crate::{Ray, Shape, Tuple, EPSILON, F};
use std::cmp::{Ord, Ordering};

pub struct Comps<'shape> {
    pub t: F,
    pub object: &'shape dyn Shape,
    pub point: Tuple,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub reflectv: Tuple,
    pub n1: F,
    pub n2: F,
}

impl<'shape> Comps<'shape> {
    #[must_use]
    pub fn schlick(&self) -> F {
        // find the cosine of the angle between the eye and normal vectors
        let mut cos = self.eyev.dot(self.normalv);

        // total internal reflection can only occur if n1 > n2
        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));

            if sin2_t > 1.0 {
                return 1.0;
            }

            // compute cosine of theta_t using trig identity
            let cos_t = F::sqrt(1.0 - sin2_t);

            // when n1 > n2, use cos(theta_t) instead
            cos = cos_t;
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection<'shape> {
    pub t: F,
    pub object: &'shape dyn Shape,
}

impl<'shape> Intersection<'shape> {
    pub fn new(t: F, object: &'shape dyn Shape) -> Intersection<'shape> {
        Self { t, object }
    }

    #[must_use]
    pub fn prepare_computations(&self, ray: Ray, intersections: &[Intersection]) -> Comps {
        let t = self.t;
        let object = self.object;

        let point = ray.position(t);
        let eyev = -ray.direction;

        let mut normalv = object.normal_at(point);
        let mut inside = false;

        if normalv.dot(eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }

        let over_point = point + normalv * EPSILON;
        let under_point = point - normalv * EPSILON;
        let reflectv = ray.direction.reflect(normalv);

        let mut n1 = 1.0;
        let mut n2 = 1.0;
        let mut containers = Vec::<&dyn Shape>::new();

        for i in intersections {
            if i == self {
                if let Some(o) = containers.last() {
                    n1 = o.props().material.refractive_index;
                }
            }

            if let Some(index) = containers.iter().position(|&o| o == i.object) {
                containers.remove(index);
            } else {
                containers.push(i.object);
            }

            if i == self {
                if let Some(o) = containers.last() {
                    n2 = o.props().material.refractive_index;
                }
                break;
            }
        }

        Comps {
            t,
            object,
            point,
            over_point,
            under_point,
            eyev,
            normalv,
            inside,
            reflectv,
            n1,
            n2,
        }
    }
}

impl Eq for Intersection<'_> {}

impl Ord for Intersection<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.partial_cmp(&other.t).expect("t is not NaN")
    }
}

impl PartialOrd for Intersection<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub trait Intersections<'a> {
    fn hit(&'a self) -> Option<&Intersection<'a>>;
}

impl<'a, I> Intersections<'a> for I
where
    I: AsRef<[Intersection<'a>]>,
{
    fn hit(&'a self) -> Option<&Intersection<'a>> {
        self.as_ref()
            .iter()
            .filter(|&i| i.t >= 0.0)
            .min_by(Ord::cmp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use itertools::izip;

    fn twosqrttwo() -> F {
        twosqrt() / 2.0
    }

    fn twosqrt() -> F {
        F::sqrt(2.0)
    }

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::default();
        let i = s.intersection(3.5);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s.as_shape());
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::default();
        let i1 = s.intersection(1.0);
        let i2 = s.intersection(2.0);
        let xs = [i1, i2];

        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::default();
        let i1 = s.intersection(1.0);
        let i2 = s.intersection(2.0);
        let xs = &[i2, i1];

        let i = xs.hit();

        assert_eq!(i, Some(&i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = s.intersection(-1.0);
        let i2 = s.intersection(1.0);
        let xs = [i2, i1];

        let i = xs.hit();

        assert_eq!(i, Some(&i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = s.intersection(-2.0);
        let i2 = s.intersection(-1.0);
        let xs = [i2, i1];
        let i = xs.hit();

        assert_eq!(i, None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::default();
        let i1 = s.intersection(5.0);
        let i2 = s.intersection(7.0);
        let i3 = s.intersection(-3.0);
        let i4 = s.intersection(2.0);
        let xs = [i1, i2, i3, i4];

        let i = xs.hit();

        assert_eq!(i, Some(&i4));
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(pt(0, 0, -5), v(0, 0, 1));
        let shape = Sphere::default();
        let i = shape.intersection(4.0);

        let comps = i.prepare_computations(r, &[i]);

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, pt(0, 0, -1));
        assert_eq!(comps.eyev, v(0, 0, -1));
        assert_eq!(comps.normalv, v(0, 0, -1));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(pt(0, 0, -5), v(0, 0, 1));
        let shape = Sphere::default();
        let i = shape.intersection(4.0);

        let comps = i.prepare_computations(r, &[i]);

        assert!(!comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(pt(0, 0, 0), v(0, 0, 1));
        let shape = Sphere::default();
        let i = shape.intersection(1.0);

        let comps = i.prepare_computations(r, &[i]);

        assert!(comps.inside);
        assert_eq!(comps.point, pt(0, 0, 1));
        assert_eq!(comps.eyev, v(0, 0, -1));
        assert_eq!(comps.normalv, v(0, 0, -1));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = ray(pt(0, 0, -5), v(0, 0, 1));
        let shape = Sphere::default().transform(Matrix::translation(0, 0, 1));
        let i = shape.intersection(5.0);
        let comps = i.prepare_computations(r, &[i]);

        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let shape = Plane::default();
        let r = ray(pt(0, 1, -1), v(0, -F::sqrt(2.0) / 2.0, F::sqrt(2.0) / 2.0));
        let i = shape.intersection(F::sqrt(2.0));
        let comps = i.prepare_computations(r, &[i]);

        assert_eq!(comps.reflectv, v(0, F::sqrt(2.0) / 2.0, F::sqrt(2.0) / 2.0));
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let mut a = glass_sphere();
        a.props_mut().transform = Matrix::scaling(2, 2, 2);
        a.props_mut().material.refractive_index = 1.5;

        let mut b = glass_sphere();
        b.props_mut().transform = Matrix::translation(0, 0, -0.25);
        b.props_mut().material.refractive_index = 2.0;

        let mut c = glass_sphere();
        c.props_mut().transform = Matrix::translation(0, 0, 0.25);
        c.props_mut().material.refractive_index = 2.5;

        let r = ray(pt(0, 0, -4), v(0, 0, 1));

        let xs = &[
            a.intersection(2.0),
            b.intersection(2.75),
            c.intersection(3.25),
            b.intersection(4.75),
            c.intersection(5.25),
            a.intersection(6.0),
        ];

        let comps = xs.iter().map(|i| i.prepare_computations(r, xs));

        let n1 = [1.0, 1.5, 2.0, 2.5, 2.5, 1.5];
        let n2 = [1.5, 2.0, 2.5, 2.5, 1.5, 1.0];

        for (c, n1, n2) in izip!(comps, n1, n2) {
            assert_eq!(c.n1, n1);
            assert_eq!(c.n2, n2);
        }
    }

    #[test]
    fn the_under_point_is_offset_bellow_the_surface() {
        let r = ray(pt(0, 0, -5), v(0, 0, 1));
        let shape = glass_sphere().transform(Matrix::translation(0, 0, 1));
        let i = shape.intersection(5.0);
        let xs = [i];
        let comps = i.prepare_computations(r, &xs);

        assert!(comps.under_point.z > EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn the_shlick_approximation_under_total_internal_reflection() {
        let shape = glass_sphere();
        let r = ray(pt(0, 0, twosqrttwo()), v(0, 1, 0));

        let xs = [
            shape.intersection(-twosqrttwo()),
            shape.intersection(twosqrttwo()),
        ];

        let comps = xs[1].prepare_computations(r, &xs);

        let reflectance = comps.schlick();
        assert_eq!(reflectance, 1.0);
    }

    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let shape = glass_sphere();
        let r = ray(pt(0, 0, 0), v(0, 1, 0));
        let xs = [shape.intersection(-1.0), shape.intersection(1.0)];
        let comps = xs[1].prepare_computations(r, &xs);

        let reflectance = comps.schlick();
        assert_fuzzy_eq!(reflectance, 0.04);
    }

    #[test]
    fn the_schlick_approximation_with_a_small_angle_and_n2_larger_than_n1() {
        let shape = glass_sphere();
        let r = ray(pt(0, 0.99, -2), v(0, 0, 1));

        let xs = [shape.intersection(1.8589)];
        let comps = xs[0].prepare_computations(r, &xs);

        let reflectance = comps.schlick();
        assert_fuzzy_eq!(reflectance, 0.48873);
    }
}
