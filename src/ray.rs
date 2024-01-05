use crate::{Color, Matrix, Tuple, F};

#[must_use]
pub fn ray(origin: Tuple, direction: Tuple) -> Ray {
    Ray::new(origin, direction)
}

#[must_use]
pub fn point_light(position: Tuple, intensity: Color) -> PointLight {
    PointLight::new(position, intensity)
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    /// # Panics
    ///
    /// origin needs to be a point and direction needs to be a vector
    #[must_use]
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

    #[must_use]
    pub fn transform(&self, matrix: Matrix<4>) -> Self {
        Self::new(matrix * self.origin, matrix * self.direction)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight {
    #[must_use]
    pub fn new(position: Tuple, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = pt(1, 2, 3);
        let direction = v(4, 5, 6);
        let r = Ray::new(origin, direction);

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(pt(2, 3, 4), v(1, 0, 0));

        assert_eq!(r.position(0), pt(2, 3, 4));
        assert_eq!(r.position(1), pt(3, 3, 4));
        assert_eq!(r.position(-1), pt(1, 3, 4));
        assert_eq!(r.position(2.5), pt(4.5, 3, 4));
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(pt(1, 2, 3), v(0, 1, 0));

        let m = Matrix::translation(3, 4, 5);
        let r2 = r.transform(m);

        assert_eq!(r2.origin, pt(4, 6, 8));
        assert_eq!(r2.direction, v(0, 1, 0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(pt(1, 2, 3), v(0, 1, 0));

        let m = Matrix::scaling(2, 3, 4);
        let r2 = r.transform(m);
        assert_eq!(r2.origin, pt(2, 6, 12));
        assert_eq!(r2.direction, v(0, 3, 0));
    }

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let position = pt(0, 0, 0);
        let intensity = color(1, 1, 1);

        let light = PointLight::new(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
