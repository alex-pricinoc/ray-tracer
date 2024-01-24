use crate::{v, FuzzyEq, Intersection, Props, Ray, Shape, Tuple, EPSILON, F, INFINITY};

#[derive(Debug)]
pub struct Cylinder {
    minimum: F,
    maximum: F,
    closed: bool,
    props: Props,
}

impl Cylinder {
    #[must_use]
    pub fn minimum(mut self, minimum: impl Into<F>) -> Self {
        self.minimum = minimum.into();

        self
    }

    #[must_use]
    pub fn maximum(mut self, maximum: impl Into<F>) -> Self {
        self.maximum = maximum.into();

        self
    }

    #[must_use]
    pub fn closed(mut self, closed: bool) -> Self {
        self.closed = closed;

        self
    }
}

fn check_cap(ray: Ray, time: F) -> bool {
    let x = ray.origin.x + time * ray.direction.x;
    let z = ray.origin.z + time * ray.direction.z;

    x.powi(2) + z.powi(2) <= 1.0
}

fn intersect_caps(cylinder: &Cylinder, ray: Ray) -> Vec<F> {
    let mut xs = vec![];

    if !cylinder.closed || ray.direction.y.fuzzy_eq(&0.0) {
        return xs;
    }

    // Check for an intersection with the lower end cap by intersecting
    // the ray with the plane at y=cylinder.minimum
    let t = (cylinder.minimum - ray.origin.y) / ray.direction.y;
    if check_cap(ray, t) {
        xs.push(t);
    }

    // Check for an intersection with the upper end cap by intersecting
    // the ray with the plane at y=cylinder.maximum
    let t = (cylinder.maximum - ray.origin.y) / ray.direction.y;
    if check_cap(ray, t) {
        xs.push(t);
    }

    xs
}

impl Default for Cylinder {
    fn default() -> Self {
        Self {
            minimum: -INFINITY,
            maximum: INFINITY,
            closed: false,
            props: Props::default(),
        }
    }
}

impl Shape for Cylinder {
    fn props(&self) -> &Props {
        &self.props
    }

    fn props_mut(&mut self) -> &mut Props {
        &mut self.props
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);
        let b = 2.0 * ray.origin.x * ray.direction.x + 2.0 * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.0;
        let disc = b.powi(2) - 4.0 * a * c;

        // ray does not intersect the cylinder
        if disc < 0.0 {
            return vec![];
        }

        let mut t0 = (-b - F::sqrt(disc)) / (2.0 * a);
        let mut t1 = (-b + F::sqrt(disc)) / (2.0 * a);
        if t0 > t1 {
            (t0, t1) = (t1, t0);
        }

        let mut xs = vec![];

        let y0 = ray.origin.y + t0 * ray.direction.y;
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(self.intersection(t0));
        }

        let y1 = ray.origin.y + t1 * ray.direction.y;
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(self.intersection(t1));
        }

        let cap_xs = intersect_caps(self, ray)
            .into_iter()
            .map(|t| self.intersection(t));

        xs.extend(cap_xs);

        xs
    }

    fn local_normal_at(&self, Tuple { x, y, z, .. }: Tuple) -> Tuple {
        let dist = x.powi(2) + z.powi(2);

        if dist < 1.0 && y > self.maximum - EPSILON {
            v(0, 1, 0)
        } else if dist < 1.0 && y < self.minimum + EPSILON {
            v(0, -1, 0)
        } else {
            v(x, 0.0, z)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn a_ray_misses_a_cylinder() {
        let cyl = Cylinder::default();

        for (origin, direction) in [
            (pt(1, 0, 0), v(0, 1, 0)),
            (pt(0, 0, 0), v(0, 1, 0)),
            (pt(0, 0, -5), v(1, 1, 1)),
        ] {
            let direction = direction.normalize();
            let r = ray(origin, direction);

            let xs = cyl.local_intersect(r);

            assert!(xs.is_empty());
        }
    }

    #[test]
    fn ray_strikes_a_cylinder() {
        let cyl = Cylinder::default();

        for (origin, direction, t0, t1) in [
            (pt(1, 0, -5), v(0, 0, 1), 5.0, 5.0),
            (pt(0, 0, -5), v(0, 0, 1), 4.0, 6.0),
            (pt(0.5, 0, -5), v(0.1, 1, 1), 6.80798, 7.08872),
        ] {
            let direction = direction.normalize();
            let r = ray(origin, direction);
            let xs = cyl.local_intersect(r);

            assert_eq!(xs.len(), 2);
            assert_fuzzy_eq!(xs[0].t, t0);
            assert_fuzzy_eq!(xs[1].t, t1);
        }
    }

    #[test]
    fn normal_vector_on_a_cylinder() {
        let cyl = Cylinder::default();

        for (point, normal) in [
            (pt(1, 0, 0), v(1, 0, 0)),
            (pt(0, 5, -1), v(0, 0, -1)),
            (pt(0, -2, 1), v(0, 0, 1)),
            (pt(-1, 1, 0), v(-1, 0, 0)),
        ] {
            let n = cyl.local_normal_at(point);
            assert_eq!(n, normal);
        }
    }

    #[test]
    fn the_default_minimum_and_maximum_for_a_cylinder() {
        let cyl = Cylinder::default();

        assert_eq!(cyl.minimum, -INFINITY);
        assert_eq!(cyl.maximum, INFINITY);
    }

    #[test]
    fn intersecting_a_constrained_cylinder() {
        let cyl = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            ..Default::default()
        };

        for (point, direction, count) in [
            (pt(0, 1.5, 0), v(0.1, 1, 0), 0),
            (pt(0, 3, -5), v(0, 0, 1), 0),
            (pt(0, 0, -5), v(0, 0, 1), 0),
            (pt(0, 2, -5), v(0, 0, 1), 0),
            (pt(0, 1, -5), v(0, 0, 1), 0),
            (pt(0, 1.5, -2), v(0, 0, 1), 2),
        ] {
            let direction = direction.normalize();
            let r = ray(point, direction);
            let xs = cyl.local_intersect(r);

            assert_eq!(xs.len(), count);
        }
    }

    #[test]
    fn the_default_closed_value() {
        let cyl = Cylinder::default();

        assert!(!cyl.closed);
    }

    #[test]
    fn intersetcion_the_caps_of_a_closed_cylinder() {
        let cyl = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            closed: true,
            ..Default::default()
        };

        for (point, direction, count) in [
            (pt(0, 3, 0), v(0, -1, 0), 2),
            (pt(0, 3, -2), v(0, -1, 2), 2),
            (pt(0, 4, -2), v(0, -1, 1), 2),
            (pt(0, 0, -2), v(0, 1, 2), 2),
            (pt(0, -1, -2), v(0, 1, 1), 2),
        ] {
            let direction = direction.normalize();
            let r = ray(point, direction);
            let xs = cyl.local_intersect(r);
            assert_eq!(xs.len(), count);
        }
    }

    #[test]
    fn the_normal_vector_on_a_cylinders_end_caps() {
        let cyl = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            closed: true,
            ..Default::default()
        };

        for (point, normal) in [
            (pt(0, 1, 0), v(0, -1, 0)),
            (pt(0.5, 1, 0), v(0, -1, 0)),
            (pt(0, 1, 0.5), v(0, -1, 0)),
            (pt(0, 2, 0), v(0, 1, 0)),
            (pt(0.5, 2, 0), v(0, 1, 0)),
            (pt(0, 2, 0.5), v(0, 1, 0)),
        ] {
            let n = cyl.local_normal_at(point);

            assert_eq!(n, normal);
        }
    }
}
