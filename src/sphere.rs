use crate::{point, Intersection, Intersections, Material, Matrix, Ray, Shape, Tuple};
use std::any::Any;

#[derive(Debug)]
pub struct Sphere {
    pub transform: Matrix<4>,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let ray = ray.transform(self.transform().inverse());

        let sphere_to_ray = ray.origin - point(0, 0, 0);

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return Intersections::default();
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let i1 = Intersection::new(t1, self);
        let i2 = Intersection::new(t2, self);

        Intersections::from([i1, i2])
    }

    pub fn transform(&self) -> Matrix<4> {
        self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = self.transform.inverse() * world_point;
        let object_normal = object_point - point(0, 0, 0);
        let mut world_normal = self.transform.inverse().transpose() * object_normal;
        world_normal.w = 0.0;

        world_normal.normalize()
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            transform: Matrix::identity(),
            material: Material::default(),
        }
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

    fn material(&self) -> &Material {
        &self.material
    }

    fn normal_at(&self, world_point: Tuple) -> Tuple {
        self.normal_at(world_point)
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
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }

    #[test]
    fn ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(point(0, 1, -5), vector(0, 0, 1));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
    }

    #[test]
    fn ray_misses_a_sphere() {
        let r = Ray::new(point(0, 2, -5), vector(0, 0, 1));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.count(), 0);
    }

    #[test]
    fn ray_originates_inside_a_sphere() {
        let r = Ray::new(point(0, 0, 0), vector(0, 0, 1));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
    }

    #[test]
    fn sphere_behind_a_ray() {
        let r = Ray::new(point(0, 0, 5), vector(0, 0, 1));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.count(), 2);
        assert_eq!(xs[0].object, &s as &dyn Shape);
        assert_eq!(xs[1].object, &s as &dyn Shape);
    }

    #[test]
    fn sphere_default_transformation() {
        let s = Sphere::new();

        assert_fuzzy_eq!(s.transform(), Matrix::identity());
    }

    #[test]
    fn changing_sphere_transformation() {
        let mut s = Sphere::new();
        let t = Matrix::translation(2, 3, 4);

        s.set_transform(t);
        assert_fuzzy_eq!(s.transform(), t)
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let mut s = Sphere::new();

        s.set_transform(Matrix::scaling(2, 2, 2));

        let xs = s.intersect(r);

        assert_eq!(xs.count(), 2);

        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_scaled_sphere_with_a_ray() {
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let mut s = Sphere::new();

        s.set_transform(Matrix::translation(5, 0, 0));

        let xs = s.intersect(r);

        assert_eq!(xs.count(), 0)
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(point(1, 0, 0));

        assert_eq!(n, vector(1, 0, 0));
    }

    #[test]
    fn the_normal_on_a_sphere_is_at_a_point_on_the_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(point(0, 1, 0));

        assert_eq!(n, vector(0, 1, 0));
    }

    #[test]
    fn the_normal_on_a_sphere_is_at_a_point_on_the_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(point(0, 0, 1));

        assert_eq!(n, vector(0, 0, 1));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::new();
        let n = s.normal_at(point(
            F::sqrt(3.0) / 3.0,
            F::sqrt(3.0) / 3.0,
            F::sqrt(3.0) / 3.0,
        ));

        assert_eq!(
            n,
            vector(F::sqrt(3.0) / 3.0, F::sqrt(3.0) / 3.0, F::sqrt(3.0) / 3.0)
        );
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere::new();
        let n = s.normal_at(point(
            F::sqrt(3.0) / 3.0,
            F::sqrt(3.0) / 3.0,
            F::sqrt(3.0) / 3.0,
        ));

        assert_eq!(n, n.normalize());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(Matrix::translation(0, 1, 0));
        let n = s.normal_at(point(0, 1.70711, -0.70711));

        assert_fuzzy_eq!(n, vector(0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = Sphere::new();
        let m = Matrix::scaling(1, 0.5, 1) * Matrix::rotation_z(PI / 5.0);
        s.set_transform(m);
        let n = s.normal_at(point(0, F::sqrt(2.0) / 2.0, -F::sqrt(2.0) / 2.0));

        assert_fuzzy_eq!(n, vector(0, 0.97014, -0.24254));
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = Sphere::new();
        let m = s.material;

        assert_eq!(m, Material::default());
    }

    #[test]
    fn a_sphere_can_be_assigned_a_material() {
        let mut s = Sphere::new();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m;

        assert_eq!(s.material, m);
    }
}
