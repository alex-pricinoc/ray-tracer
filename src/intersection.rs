use crate::{Shape, F};
use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection<'shape> {
    pub t: F,
    pub object: &'shape dyn Shape,
}

impl<'shape> Intersection<'shape> {
    pub fn new(t: F, object: &'shape dyn Shape) -> Intersection<'shape> {
        Self { t, object }
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

#[derive(Debug, Default)]
pub struct Intersections<'shape>(Vec<Intersection<'shape>>);

impl Intersections<'_> {
    pub fn hit(&self) -> Option<&Intersection> {
        self.0.iter().find(|&i| i.t >= 0.)
    }

    pub fn count(&self) -> usize {
        self.len()
    }
}

impl<'shape, const N: usize> From<[Intersection<'shape>; N]> for Intersections<'shape> {
    fn from(xs: [Intersection<'shape>; N]) -> Self {
        let mut xs = Vec::from(xs);
        xs.sort_unstable();

        Self(xs)
    }
}

impl<'shape> Deref for Intersections<'shape> {
    type Target = Vec<Intersection<'shape>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Intersections<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
        let xs = Intersections::from([i1, i2]);

        assert_eq!(xs.count(), 2);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::from([i2, i1]);

        let i = xs.hit();
        assert_eq!(i, Some(&i1))
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = Intersections::from([i2, i1]);

        let i = xs.hit();
        assert_eq!(i, Some(&i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = Intersections::from([i2, i1]);
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
        let xs = Intersections::from([i1, i2, i3, i4]);

        let i = xs.hit();

        assert_eq!(i, Some(&i4));
    }
}
