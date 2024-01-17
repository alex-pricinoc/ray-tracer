use crate::{v, Intersection, Material, Matrix, Props, Ray, Shape, Tuple, F};
use std::any::Any;

#[derive(Debug, Default)]
pub struct Cube {
    props: Props,
}

pub fn check_axis(origin: F, direction: F) -> (F, F) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let tmin = tmin_numerator / direction;
    let tmax = tmax_numerator / direction;

    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}

impl Cube {
    #[must_use]
    pub fn transform(mut self, transform: Matrix<4>) -> Self {
        self.props.transform = transform;

        self
    }

    #[must_use]
    pub fn material(mut self, material: Material) -> Self {
        self.props.material = material;

        self
    }
}

impl Shape for Cube {
    fn as_shape(&self) -> &dyn Shape {
        self
    }

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
        let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z);

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        if tmin > tmax {
            return vec![];
        }

        let i1 = self.intersection(tmin);
        let i2 = self.intersection(tmax);

        vec![i1, i2]
    }

    fn local_normal_at(&self, Tuple { x, y, z, .. }: Tuple) -> Tuple {
        let maxc = x.abs().max(y.abs()).max(z.abs());

        if maxc == x.abs() {
            return v(x, 0, 0);
        } else if maxc == y.abs() {
            return v(0, y, 0);
        }

        v(0.0, 0.0, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn a_ray_intersects_a_cube() {
        let c = Cube::default();

        let directions = [
            /*      +x  */ (pt(5, 0.5, 0), v(-1, 0, 0), 4, 6),
            /*      -x  */ (pt(-5, 0.5, 0), v(1, 0, 0), 4, 6),
            /*      +y  */ (pt(0.5, 5, 0), v(0, -1, 0), 4, 6),
            /*      -y  */ (pt(0.5, -5, 0), v(0, 1, 0), 4, 6),
            /*      +z  */ (pt(0.5, 0, 5), v(0, 0, -1), 4, 6),
            /*      -z  */ (pt(0.5, 0, -5), v(0, 0, 1), 4, 6),
            /*  inside  */ (pt(0, 0.5, 0), v(0, 0, 1), -1, 1),
        ];

        for (origin, direction, t1, t2) in directions {
            let r = ray(origin, direction);
            let xs = c.local_intersect(r);

            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, t1.into());
            assert_eq!(xs[1].t, t2.into());
        }
    }

    #[test]
    fn ray_misses_a_cube() {
        let c = Cube::default();

        for (origin, direction) in [
            (pt(-2, 0, 0), v(0.2673, 0.5345, 0.8018)),
            (pt(0, -2, 0), v(0.8018, 0.2673, 0.5345)),
            (pt(0, 0, -2), v(0.5345, 0.8018, 0.2673)),
            (pt(2, 0, 2), v(0, 0, -1)),
            (pt(0, 2, 2), v(0, -1, 0)),
            (pt(2, 2, 0), v(-1, 0, 0)),
        ] {
            let r = ray(origin, direction);
            let xs = c.local_intersect(r);

            assert!(xs.is_empty());
        }
    }

    #[test]
    fn the_normal_on_a_surface_of_a_cube() {
        let c = Cube::default();

        for (point, normal) in [
            (pt(1, 0.5, -0.8), v(1, 0, 0)),
            (pt(-1, -0.2, 0.9), v(-1, 0, 0)),
            (pt(-0.4, 1, -0.1), v(0, 1, 0)),
            (pt(0.3, -1, -0.7), v(0, -1, 0)),
            (pt(-0.6, 0.3, 1), v(0, 0, 1)),
            (pt(0.4, 0.4, -1), v(0, 0, -1)),
            (pt(1, 1, 1), v(1, 0, 0)),
            (pt(-1, -1, -1), v(-1, 0, 0)),
        ] {
            let n = c.local_normal_at(point);

            assert_eq!(normal, n);
        }
    }
}
