use crate::{pt, Intersection, Material, Matrix, Props, Ray, Shape, Tuple};
use std::any::Any;

#[derive(Debug, Default)]
pub struct Sphere {
    props: Props,
}

impl Sphere {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

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

impl Shape for Sphere {
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
        let sphere_to_ray = ray.origin - pt(0, 0, 0);

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let i1 = Intersection::new(t1, self);
        let i2 = Intersection::new(t2, self);

        vec![i1, i2]
    }

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        point - pt(0, 0, 0)
    }
}

impl From<Sphere> for Box<dyn Shape> {
    fn from(value: Sphere) -> Box<dyn Shape> {
        Box::new(value)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::approx_constant)]

    use super::*;
    use crate::*;

    #[test]
    fn ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(pt(0, 0, -5), v(0, 0, 1));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }

    #[test]
    fn ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(pt(0, 1, -5), v(0, 0, 1));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
    }

    #[test]
    fn ray_misses_a_sphere() {
        let r = Ray::new(pt(0, 2, -5), v(0, 0, 1));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_a_sphere() {
        let r = Ray::new(pt(0, 0, 0), v(0, 0, 1));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
    }

    #[test]
    fn sphere_behind_a_ray() {
        let r = Ray::new(pt(0, 0, 5), v(0, 0, 1));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(pt(0, 0, -5), v(0, 0, 1));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, &s as &dyn Shape);
        assert_eq!(xs[1].object, &s as &dyn Shape);
    }

    #[test]
    fn sphere_default_transformation() {
        let s = Sphere::new();

        assert_eq!(s.props.transform, Matrix::identity());
    }

    #[test]
    fn changing_sphere_transformation() {
        let t = Matrix::translation(2, 3, 4);
        let s = Sphere::new().transform(t);

        assert_eq!(s.props.transform, t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(pt(0, 0, -5), v(0, 0, 1));
        let s = Sphere::new().transform(Matrix::scaling(2, 2, 2));

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);

        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_scaled_sphere_with_a_ray() {
        let r = Ray::new(pt(0, 0, -5), v(0, 0, 1));
        let s = Sphere::new().transform(Matrix::translation(5, 0, 0));

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(pt(1, 0, 0));

        assert_eq!(n, v(1, 0, 0));
    }

    #[test]
    fn the_normal_on_a_sphere_is_at_a_point_on_the_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(pt(0, 1, 0));

        assert_eq!(n, v(0, 1, 0));
    }

    #[test]
    fn the_normal_on_a_sphere_is_at_a_point_on_the_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(pt(0, 0, 1));

        assert_eq!(n, v(0, 0, 1));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_pt() {
        let s = Sphere::new();
        let n = s.normal_at(pt(
            F::sqrt(3.0) / 3.0,
            F::sqrt(3.0) / 3.0,
            F::sqrt(3.0) / 3.0,
        ));

        assert_eq!(
            n,
            v(F::sqrt(3.0) / 3.0, F::sqrt(3.0) / 3.0, F::sqrt(3.0) / 3.0)
        );
    }

    #[test]
    fn the_normal_is_a_normalized_v() {
        let s = Sphere::new();
        let n = s.normal_at(pt(
            F::sqrt(3.0) / 3.0,
            F::sqrt(3.0) / 3.0,
            F::sqrt(3.0) / 3.0,
        ));

        assert_eq!(n, n.normalize());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let s = Sphere::new().transform(Matrix::translation(0, 1, 0));
        let n = s.normal_at(pt(0, 1.70711, -0.70711));

        assert_eq!(n, v(0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let s = Sphere::new().transform(Matrix::scaling(1, 0.5, 1) * Matrix::rotation_z(PI / 5.0));
        let n = s.normal_at(pt(0, F::sqrt(2.0) / 2.0, -F::sqrt(2.0) / 2.0));

        assert_eq!(n, v(0, 0.97014, -0.24254));
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = Sphere::new();
        let m = s.props.material;

        assert_eq!(m, Material::default());
    }

    #[test]
    fn a_sphere_can_be_assigned_a_material() {
        let mut m = Material::new();
        m.ambient = 1.0;

        let s = Sphere::new().material(m);

        assert_eq!(s.props.material, m);
    }
}
