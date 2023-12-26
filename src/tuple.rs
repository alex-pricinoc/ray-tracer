use crate::FuzzyEq;
use crate::F;
use std::fmt;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

pub fn point(x: impl Into<F>, y: impl Into<F>, z: impl Into<F>) -> Tuple {
    Tuple::point(x.into(), y.into(), z.into())
}

pub fn vector(x: impl Into<F>, y: impl Into<F>, z: impl Into<F>) -> Tuple {
    Tuple::vector(x.into(), y.into(), z.into())
}

impl<A: Into<F>, B: Into<F>, C: Into<F>, D: Into<F>> From<(A, B, C, D)> for Tuple {
    fn from(t: (A, B, C, D)) -> Self {
        Tuple::new(t.0.into(), t.1.into(), t.2.into(), t.3.into())
    }
}

#[must_use]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tuple {
    pub x: F,
    pub y: F,
    pub z: F,
    pub w: F,
}

impl Tuple {
    fn new(x: F, y: F, z: F, w: F) -> Self {
        Self { x, y, z, w }
    }

    fn point(x: F, y: F, z: F) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    fn vector(x: F, y: F, z: F) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn is_point(&self) -> bool {
        self.w.fuzzy_eq(1.0)
    }

    pub fn is_vector(&self) -> bool {
        self.w.fuzzy_eq(0.0)
    }

    pub fn reflect(&self, normal: Tuple) -> Self {
        *self - normal * 2.0 * self.dot(normal)
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
        Self::from((
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        ))
    }
}

impl Sub<Self> for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::from((
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        ))
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from((-self.x, -self.y, -self.z, -self.w))
    }
}

impl Mul<F> for Tuple {
    type Output = Self;

    fn mul(self, other: F) -> Self::Output {
        Self::from((
            self.x * other,
            self.y * other,
            self.z * other,
            self.w * other,
        ))
    }
}

impl Div<F> for Tuple {
    type Output = Self;

    fn div(self, other: F) -> Self::Output {
        Self::from((
            self.x / other,
            self.y / other,
            self.z / other,
            self.w / other,
        ))
    }
}

impl Index<usize> for Tuple {
    type Output = F;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Out of bound index used {index}"),
        }
    }
}
impl IndexMut<usize> for Tuple {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Out of bound index used {index}"),
        }
    }
}

impl Tuple {
    /// Magnitude represents the length of the vector.
    /// It’s how far you would travel in a straight line if you
    /// were to walk from one end of the vector to the other.
    ///
    /// Vectors with magnitude of 1 are called `unit vectors`.
    fn magnitude(self) -> F {
        F::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2))
    }

    /// Normalization is the process of taking an arbitrary vector and
    /// converting it into a unit vector. It will keep your calculations
    /// anchored relative to a common scale (the unit vector).
    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }

    /// Dot product (or scalar product or inner product)
    /// Takes two vectors and returns a scalar value.
    /// Used when intersecting rays with objects.
    /// The smaller the dot product, the larger the angle between the vectors.
    /// A dot product of 1 means vectors are identical.
    /// -1 means they point in opposite directions.
    /// If two vectors are unit vectors, the dot product is the cosine of the
    /// angle between them.
    /// For more info: http://betterexplained.com/articles/vector-calculus-understanding-the-dot-product
    pub fn dot(self, other: Self) -> F {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(self, other: Self) -> Self {
        assert!(
            self.is_vector() && other.is_vector(),
            "Cross product can only be calculated for two vectors."
        );

        vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc = if self.is_point() { "point" } else { "vector" };

        write!(f, "{}({:.1}, {:.1}, {:.1})", desc, self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_says_it_is_a_point() {
        let point = point(4.3, -4.2, 3.1);
        assert!(point.is_point());
        assert!(!point.is_vector());
    }

    #[test]
    fn point_says_it_is_a_vector() {
        let vector = vector(4.3, -4.2, 3.1);
        assert!(vector.is_vector());
        assert!(!vector.is_point());
    }

    #[test]
    fn point_creates_tuple_with_w_1() {
        let point = point(4.0, -4.0, 3.0);

        assert_eq!(point.x, 4.0);
        assert_eq!(point.y, -4.0);
        assert_eq!(point.z, 3.0);
        assert_eq!(point.w, 1.0);
    }

    #[test]
    fn vector_creates_tuple_with_w_0() {
        let vector = vector(4.0, -4.0, 3.0);

        assert_eq!(vector.x, 4.0);
        assert_eq!(vector.y, -4.0);
        assert_eq!(vector.z, 3.0);
        assert_eq!(vector.w, 0.0);
    }

    #[test]
    fn adding_two_tuples() {
        let tuple_one = Tuple::from((3.0, -2.0, 5.0, 1.0));
        let tuple_two = Tuple::from((-2.0, 3.0, 1.0, 0.0));

        assert_fuzzy_eq!(tuple_one + tuple_two, Tuple::from((1.0, 1.0, 6.0, 1.0)));
    }

    #[test]
    fn substracting_two_points() {
        let point_one = point(3.0, 2.0, 1.0);
        let point_two = point(5.0, 6.0, 7.0);

        assert_fuzzy_eq!(point_one - point_two, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);

        assert_fuzzy_eq!(p - v, point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);

        assert_fuzzy_eq!(v1 - v2, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_the_zero_vector() {
        let zero = vector(0.0, 0.0, 0.0);
        let v = vector(1.0, -2.0, 3.0);

        assert_fuzzy_eq!(zero - v, vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negating_tuple() {
        let a = Tuple::from((1.0, -2.0, 3.0, -4.0));

        assert_fuzzy_eq!(-a, Tuple::from((-1.0, 2.0, -3.0, 4.0)));
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = Tuple::from((1.0, -2.0, 3.0, -4.0));

        assert_fuzzy_eq!(a * 3.5, Tuple::from((3.5, -7.0, 10.5, -14.0)));
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a = Tuple::from((1.0, -2.0, 3.0, -4.0));

        assert_fuzzy_eq!(a * 0.5, Tuple::from((0.5, -1.0, 1.5, -2.0)));
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = Tuple::from((1.0, -2.0, 3.0, -4.0));

        assert_fuzzy_eq!(a / 2.0, Tuple::from((0.5, -1.0, 1.5, -2.0)));
    }

    #[test]
    fn computing_the_magnitude_of_vector_1() {
        let v = vector(1.0, 0.0, 0.0);

        assert_fuzzy_eq!(v.magnitude(), 1.0)
    }

    #[test]
    fn computing_the_magnitude_of_vector_2() {
        let v = vector(0.0, 1.0, 0.0);

        assert_fuzzy_eq!(v.magnitude(), 1.0)
    }

    #[test]
    fn computing_the_magnitude_of_vector_3() {
        let v = vector(0.0, 0.0, 1.0);

        assert_fuzzy_eq!(v.magnitude(), 1.0)
    }

    #[test]
    fn computing_the_magnitude_of_vector_4() {
        let v = vector(1.0, 2.0, 3.0);

        assert_fuzzy_eq!(v.magnitude(), F::sqrt(14.0))
    }

    #[test]
    fn computing_the_magnitude_of_vector_5() {
        let v = vector(-1.0, -2.0, -3.0);

        assert_fuzzy_eq!(v.magnitude(), F::sqrt(14.0))
    }

    #[test]
    fn normalize_vector() {
        let v = vector(4.0, 0.0, 0.0);
        let expected = vector(1.0, 0.0, 0.0);

        assert_fuzzy_eq!(v.normalize(), expected);
    }

    #[test]
    fn normalize_vector_2() {
        let v = vector(1.0, 2.0, 3.0);

        assert_fuzzy_eq!(v.normalize(), vector(0.26726, 0.53452, 0.80178));
    }

    #[test]
    fn magnitude_of_a_normalized_vector() {
        let v = vector(1.0, 2.0, 3.0);
        let norm = v.normalize();

        assert_fuzzy_eq!(norm.magnitude(), 1.0);
    }

    #[test]
    fn the_dot_product_of_two_tuples() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);

        assert_fuzzy_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);

        assert_fuzzy_eq!(a.cross(b), vector(-1.0, 2.0, -1.0));
        assert_fuzzy_eq!(b.cross(a), vector(1.0, -2.0, 1.0));
    }

    #[test]
    fn reflecting_a_vector_aproaching_at_45_deg() {
        let v = vector(1, -1, 0);
        let n = vector(0, 1, 0);
        let r = v.reflect(n);

        assert_fuzzy_eq!(r, vector(1, 1, 0));
    }

    #[test]
    fn reflecting_a_vector_of_a_slanted_surface() {
        let v = vector(0, -1, 0);
        let n = vector(F::sqrt(2.0) / 2.0, F::sqrt(2.0) / 2.0, 0);
        let r = v.reflect(n);

        assert_fuzzy_eq!(r, vector(1, 0, 0));
    }
}
