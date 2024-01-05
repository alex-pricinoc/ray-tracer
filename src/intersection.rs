use crate::{Ray, Shape, Tuple, EPSILON, F};
use std::cmp::{Ord, Ordering};

pub struct Comps<'shape> {
    pub t: F,
    pub object: &'shape dyn Shape,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
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
    pub fn prepare_computations(&self, ray: Ray) -> Comps {
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

        Comps {
            t,
            object,
            point,
            over_point,
            eyev,
            normalv,
            inside,
        }
    }
}

impl PartialEq<F> for Intersection<'_> {
    fn eq(&self, other: &F) -> bool {
        self.t == *other
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

pub trait Intersections {
    fn hit(&self) -> Option<&Intersection<'_>>;
}

impl Intersections for Vec<Intersection<'_>> {
    fn hit(&self) -> Option<&Intersection<'_>> {
        self.iter().filter(|&i| i.t >= 0.0).min_by(Ord::cmp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &s as &dyn Shape);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        #[allow(clippy::useless_vec)]
        let xs = vec![i1, i2];

        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i2, i1];

        let i = xs.hit();
        assert_eq!(i, Some(&i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = vec![i2, i1];

        let i = xs.hit();
        assert_eq!(i, Some(&i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = vec![i2, i1];
        let i = xs.hit();

        assert_eq!(i, None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2, i3, i4];

        let i = xs.hit();

        assert_eq!(i, Some(&i4));
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(pt(0, 0, -5), v(0, 0, 1));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape as &dyn Shape);

        let comps = i.prepare_computations(r);

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, pt(0, 0, -1));
        assert_eq!(comps.eyev, v(0, 0, -1));
        assert_eq!(comps.normalv, v(0, 0, -1));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(pt(0, 0, -5), v(0, 0, 1));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape as &dyn Shape);

        let comps = i.prepare_computations(r);
        assert!(!comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(pt(0, 0, 0), v(0, 0, 1));
        let shape = Sphere::new();
        let i = Intersection::new(1.0, &shape as &dyn Shape);

        let comps = i.prepare_computations(r);
        assert!(comps.inside);
        assert_eq!(comps.point, pt(0, 0, 1));
        assert_eq!(comps.eyev, v(0, 0, -1));
        assert_eq!(comps.normalv, v(0, 0, -1));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = ray(pt(0, 0, -5), v(0, 0, 1));
        let shape = Sphere::new().transform(Matrix::translation(0, 0, 1));
        let i = Intersection::new(5.0, &shape);
        let comps = i.prepare_computations(r);
        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
