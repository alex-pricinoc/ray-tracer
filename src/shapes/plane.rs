use crate::{v, Intersection, Props, Ray, Shape, Tuple, EPSILON};

#[must_use]
pub fn glass() -> Plane {
    let mut s = Plane::default();
    s.props.material.transparency = 1.0;
    s.props.material.refractive_index = 1.5;
    s
}

#[derive(Debug, Default)]
pub struct Plane {
    props: Props,
}

impl Shape for Plane {
    fn props(&self) -> &Props {
        &self.props
    }

    fn props_mut(&mut self) -> &mut Props {
        &mut self.props
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        if ray.direction.y.abs() < EPSILON {
            return vec![];
        }

        let t = -ray.origin.y / ray.direction.y;

        let i1 = Intersection::new(t, self);

        vec![i1]
    }

    fn local_normal_at(&self, _point: Tuple) -> Tuple {
        v(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::default();

        let n1 = p.local_normal_at(pt(0, 0, 0));
        let n2 = p.local_normal_at(pt(10, 0, -10));
        let n3 = p.local_normal_at(pt(-5, 0, 150));

        assert_eq!(n1, v(0, 1, 0));
        assert_eq!(n2, v(0, 1, 0));
        assert_eq!(n3, v(0, 1, 0));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Plane::default();
        let r = ray(pt(0, 10, 0), v(0, 0, 1));
        let xs = p.local_intersect(r);

        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Plane::default();
        let r = ray(pt(0, 0, 0), v(0, 0, 1));
        let xs = p.local_intersect(r);

        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Plane::default();
        let r = ray(pt(0, 1, 0), v(0, -1, 0));
        let xs = p.local_intersect(r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, &p as &dyn Shape);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane::default();
        let r = ray(pt(0, -1, 0), v(0, 1, 0));
        let xs = p.local_intersect(r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, &p as &dyn Shape);
    }
}
