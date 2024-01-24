use crate::{v, FuzzyEq, Intersection, Props, Ray, Shape, Tuple, EPSILON, F, INFINITY};

#[derive(Debug)]
pub struct Cone {
    minimum: F,
    maximum: F,
    closed: bool,
    props: Props,
}

impl Cone {
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

impl Default for Cone {
    fn default() -> Self {
        Self {
            minimum: -INFINITY,
            maximum: INFINITY,
            closed: false,
            props: Props::default(),
        }
    }
}

fn check_cap(ray: Ray, time: F, radius: F) -> bool {
    let x = ray.origin.x + time * ray.direction.x;
    let z = ray.origin.z + time * ray.direction.z;
    x.powi(2) + z.powi(2) <= radius
}

fn intersect_caps(cone: &Cone, ray: Ray) -> Vec<F> {
    let mut xs = vec![];
    if !cone.closed || ray.direction.y.fuzzy_eq(&0.0) {
        return xs;
    }

    // Check for an intersection with the lower end cap by intersecting
    // the ray with the plane at y=cylinder.minimum
    let t = (cone.minimum - ray.origin.y) / ray.direction.y;
    if check_cap(ray, t, cone.minimum.abs()) {
        xs.push(t);
    }

    // Check for an intersection with the upper end cap by intersecting
    // the ray with the plane at y=cylinder.maximum
    let t = (cone.maximum - ray.origin.y) / ray.direction.y;
    if check_cap(ray, t, cone.maximum.abs()) {
        xs.push(t);
    }

    xs
}

impl Shape for Cone {
    fn props(&self) -> &Props {
        &self.props
    }

    fn props_mut(&mut self) -> &mut Props {
        &mut self.props
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let Tuple {
            x: ox,
            y: oy,
            z: oz,
            ..
        } = ray.origin;
        let Tuple {
            x: dx,
            y: dy,
            z: dz,
            ..
        } = ray.direction;

        let a = dx.powi(2) - dy.powi(2) + dz.powi(2);
        let b = 2.0 * ox * dx - 2.0 * oy * dy + 2.0 * oz * dz;
        let c = ox.powi(2) - oy.powi(2) + oz.powi(2);

        let a_zero = a.fuzzy_eq(&0.0);
        let b_zero = b.fuzzy_eq(&0.0);

        if a_zero && b_zero {
            return vec![];
        }

        let mut xs = vec![];
        if a_zero {
            let t = -c / (2.0 * b);
            xs.push(self.intersection(t));
        } else {
            let disc = b.powi(2) - 4.0 * a * c;
            if disc < 0.0 {
                return vec![];
            }

            let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
            let mut t1 = (-b + disc.sqrt()) / (2.0 * a);
            if t0 > t1 {
                (t0, t1) = (t1, t0);
            }

            let y0 = ray.origin.y + t0 * ray.direction.y;
            if self.minimum < y0 && y0 < self.maximum {
                xs.push(self.intersection(t0));
            }

            let y1 = ray.origin.y + t1 * ray.direction.y;
            if self.minimum < y1 && y1 < self.maximum {
                xs.push(self.intersection(t1));
            }
        }

        let caps_xs = intersect_caps(self, ray)
            .into_iter()
            .map(|t| self.intersection(t));
        xs.extend(caps_xs);

        xs
    }

    fn local_normal_at(&self, point @ Tuple { x, y, z, .. }: Tuple) -> Tuple {
        let dist = x.powi(2) + z.powi(2);
        if dist < 1.0 && y > self.maximum - EPSILON {
            v(0, 1, 0)
        } else if dist < 1.0 && y < self.minimum + EPSILON {
            v(0, -1, 0)
        } else {
            let y = (x.powi(2) + z.powi(2)).sqrt();
            let y = if point.y > 0.0 { -y } else { y };
            v(x, y, z)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test() {
        let shape = Cone::default();

        for (origin, direction, t0, t1) in [
            (pt(0, 0, -5), v(0, 0, 1), 5.0, 5.0),
            (pt(0, 0, -5), v(1, 1, 1), 8.66025, 8.66025),
            (pt(1, 1, -5), v(-0.5, -1, 1), 4.55006, 49.44994),
        ] {
            let direction = direction.normalize();
            let r = ray(origin, direction);
            let xs = shape.local_intersect(r);

            assert_eq!(xs.len(), 2);
            assert_fuzzy_eq!(xs[0].t, t0);
            assert_fuzzy_eq!(xs[1].t, t1);
        }
    }

    #[test]
    fn intersectin_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let shape = Cone::default();
        let direction = v(0, 1, 1).normalize();
        let r = ray(pt(0, 0, -1), direction);
        let xs = shape.local_intersect(r);

        assert_eq!(xs.len(), 1);
        assert_fuzzy_eq!(xs[0].t, 0.35355);
    }

    #[test]
    fn intersectin_a_cones_end_caps() {
        let shape = Cone {
            minimum: -0.5,
            maximum: 0.5,
            closed: true,
            ..Default::default()
        };

        for (origin, direction, count) in [
            (pt(0, 0, -5), v(0, 1, 0), 0),
            (pt(0.0, 0.0, -0.25), v(0, 1, 1), 2),
            (pt(0.0, 0.0, -0.25), v(0, 1, 0), 4),
        ] {
            let direction = direction.normalize();
            let r = ray(origin, direction);
            let xs = shape.local_intersect(r);

            assert_eq!(xs.len(), count);
        }
    }

    #[test]
    fn computing_the_normal_vector_on_a_cone() {
        let shape = Cone::default();

        for (point, normal) in [
            (pt(0, 0, 0), v(0, 0, 0)),
            (pt(1, 1, 1), v(1, -F::sqrt(2.0), 1)),
            (pt(-1, -1, 0), v(-1, 1, 0)),
        ] {
            let n = shape.local_normal_at(point);
            assert_eq!(n, normal);
        }
    }
}
