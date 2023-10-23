use crate::F;

use crate::fuzzy_eq::*;

use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tuple {
    pub x: F,
    pub y: F,
    pub z: F,
    pub w: F,
}

impl Tuple {
    pub fn new(x: F, y: F, z: F, w: F) -> Self {
        Self { x, y, z, w }
    }

    pub fn point(x: F, y: F, z: F) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn vector(x: F, y: F, z: F) -> Self {
        Self { x, y, z, w: 0.0 }
    }
}

impl Tuple {
    pub fn is_point(&self) -> bool {
        self.w.fuzzy_eq(1.0)
    }

    pub fn is_vector(&self) -> bool {
        self.w.fuzzy_eq(0.0)
    }
}

impl FuzzyEq<Tuple> for Tuple {
    fn fuzzy_eq(&self, other: Self) -> bool {
        self.x.fuzzy_eq(other.x)
            && self.y.fuzzy_eq(other.y)
            && self.z.fuzzy_eq(other.z)
            && self.w.fuzzy_eq(other.w)
    }
}

impl Add<Self> for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Tuple::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl Sub<Self> for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Tuple::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<F> for Tuple {
    type Output = Self;

    fn mul(self, other: F) -> Self::Output {
        Tuple::new(
            self.x * other,
            self.y * other,
            self.z * other,
            self.w * other,
        )
    }
}

impl Div<F> for Tuple {
    type Output = Self;

    fn div(self, other: F) -> Self::Output {
        Tuple::new(
            self.x / other,
            self.y / other,
            self.z / other,
            self.w / other,
        )
    }
}

impl Tuple {
    fn magnitude(&self) -> F {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    fn dot(&self, other: Self) -> F {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    fn cross(&self, other: Self) -> Self {
        assert!(
            self.is_vector() && other.is_vector(),
            "Cross product can only be calculated for two vectors."
        );

        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_says_it_is_a_point() {
        let point = Tuple::point(4.3, -4.2, 3.1);
        assert!(point.is_point());
        assert!(!point.is_vector());
    }

    #[test]
    fn point_says_it_is_a_vector() {
        let vector = Tuple::vector(4.3, -4.2, 3.1);
        assert!(vector.is_vector());
        assert!(!vector.is_point());
    }

    #[test]
    fn point_creates_tuple_with_w_1() {
        let point = Tuple::point(4.0, -4.0, 3.0);

        assert_eq!(point.x, 4.0);
        assert_eq!(point.y, -4.0);
        assert_eq!(point.z, 3.0);
        assert_eq!(point.w, 1.0);
    }

    #[test]
    fn vector_creates_tuple_with_w_0() {
        let vector = Tuple::vector(4.0, -4.0, 3.0);

        assert_eq!(vector.x, 4.0);
        assert_eq!(vector.y, -4.0);
        assert_eq!(vector.z, 3.0);
        assert_eq!(vector.w, 0.0);
    }

    #[test]
    fn adding_two_tuples() {
        let tuple_one = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let tuple_two = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        let expected_tuple = Tuple::new(1.0, 1.0, 6.0, 1.0);

        assert_fuzzy_eq!(tuple_one + tuple_two, expected_tuple);
    }

    #[test]
    fn substracting_two_points() {
        let point_one = Tuple::point(3.0, 2.0, 1.0);
        let point_two = Tuple::point(5.0, 6.0, 7.0);

        let expected_tuple = Tuple::vector(-2.0, -4.0, -6.0);

        assert_fuzzy_eq!(point_one - point_two, expected_tuple);
    }

    #[test]
    fn subtracting_vector_from_point() {
        let point = Tuple::point(3.0, 2.0, 1.0);
        let vector = Tuple::vector(5.0, 6.0, 7.0);

        let expected = Tuple::point(-2.0, -4.0, -6.0);

        assert_fuzzy_eq!(point - vector, expected);
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);

        let expected = Tuple::vector(-2.0, -4.0, -6.0);

        assert_fuzzy_eq!(v1 - v2, expected);
    }

    #[test]
    fn subtracting_vector_from_the_zero_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);

        let expected = Tuple::vector(-1.0, 2.0, -3.0);

        assert_fuzzy_eq!(zero - v, expected);
    }

    #[test]
    fn negating_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let expected = Tuple::new(-1.0, 2.0, -3.0, 4.0);

        assert_fuzzy_eq!(-a, expected);
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let expected = Tuple::new(3.5, -7.0, 10.5, -14.0);

        assert_fuzzy_eq!(a * 3.5, expected);
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let expected = Tuple::new(0.5, -1.0, 1.5, -2.0);

        assert_fuzzy_eq!(a * 0.5, expected);
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let expected = Tuple::new(0.5, -1.0, 1.5, -2.0);

        assert_fuzzy_eq!(a / 2.0, expected);
    }

    #[test]
    fn computing_the_magnitude_of_vector_1() {
        let v = Tuple::vector(1.0, 0.0, 0.0);
        assert_fuzzy_eq!(v.magnitude(), 1.0)
    }

    #[test]
    fn computing_the_magnitude_of_vector_2() {
        let v = Tuple::vector(0.0, 1.0, 0.0);

        assert_fuzzy_eq!(v.magnitude(), 1.0)
    }

    #[test]
    fn computing_the_magnitude_of_vector_3() {
        let v = Tuple::vector(0.0, 0.0, 1.0);

        assert_fuzzy_eq!(v.magnitude(), 1.0)
    }

    #[test]
    fn computing_the_magnitude_of_vector_4() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        assert_fuzzy_eq!(v.magnitude(), F::sqrt(14.0))
    }

    #[test]
    fn computing_the_magnitude_of_vector_5() {
        let v = Tuple::vector(-1.0, -2.0, -3.0);

        dbg!(F::sqrt(14.0));

        assert_fuzzy_eq!(v.magnitude(), F::sqrt(14.0))
    }

    #[test]
    fn normalize_vector() {
        let v = Tuple::vector(4.0, 0.0, 0.0);
        let expected = Tuple::vector(1.0, 0.0, 0.0);

        assert_fuzzy_eq!(v.normalize(), expected);
    }

    #[test]
    fn normalize_vector_2() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        let expected = Tuple::vector(0.26726, 0.53452, 0.80178);

        assert_fuzzy_eq!(v.normalize(), expected);
    }

    #[test]
    fn magnitude_of_a_normalized_vector() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        let norm = v.normalize();

        assert_fuzzy_eq!(norm.magnitude(), 1.0);
    }

    #[test]
    fn the_dot_product_of_two_tuples() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert_fuzzy_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert_fuzzy_eq!(a.cross(b), Tuple::vector(-1.0, 2.0, -1.0));
        assert_fuzzy_eq!(b.cross(a), Tuple::vector(1.0, -2.0, 1.0));
    }
}
