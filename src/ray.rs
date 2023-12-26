use crate::Matrix;
use crate::Tuple;
use crate::F;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        assert!(
            origin.is_point() && direction.is_vector(),
            "origin needs to be a point and direction needs to be a vector!"
        );

        Self { origin, direction }
    }

    pub fn position(&self, t: impl Into<F>) -> Tuple {
        self.origin + self.direction * t.into()
    }

    pub fn transform(&self, matrix: Matrix<4>) -> Self {
        Self::new(matrix * self.origin, matrix * self.direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = point(1, 2, 3);
        let direction = vector(4, 5, 6);
        let r = Ray::new(origin, direction);

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction)
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(point(2, 3, 4), vector(1, 0, 0));

        assert_eq!(r.position(0), point(2, 3, 4));
        assert_eq!(r.position(1), point(3, 3, 4));
        assert_eq!(r.position(-1), point(1, 3, 4));
        assert_eq!(r.position(2.5), point(4.5, 3, 4));
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(point(1, 2, 3), vector(0, 1, 0));

        let m = Matrix::translation(3, 4, 5);
        let r2 = r.transform(m);

        assert_eq!(r2.origin, point(4, 6, 8));
        assert_eq!(r2.direction, vector(0, 1, 0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(point(1, 2, 3), vector(0, 1, 0));

        let m = Matrix::scaling(2, 3, 4);
        let r2 = r.transform(m);
        assert_eq!(r2.origin, point(2, 6, 12));
        assert_eq!(r2.direction, vector(0, 3, 0));
    }
}
