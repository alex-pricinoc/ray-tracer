use crate::{
    color, pt, ray, Color, Comps, Intersection, Intersections, Material, Matrix, PointLight, Ray,
    Shape, Sphere, Tuple, BLACK,
};

pub struct World {
    pub objects: Vec<Box<dyn Shape>>,
    pub lights: Vec<PointLight>,
}

impl World {
    #[must_use]
    pub fn new() -> Self {
        World {
            objects: vec![],
            lights: vec![],
        }
    }

    #[must_use]
    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        self.objects.iter().flat_map(|o| o.intersect(ray)).collect()
    }

    pub fn shade_hit(&self, comps: &Comps, remaining: u8) -> Color {
        let surface = self
            .lights
            .iter()
            .map(|&l| {
                comps.object.props().material.lighting(
                    comps.object,
                    l,
                    comps.point,
                    comps.eyev,
                    comps.normalv,
                    self.is_shadowed(l, comps.over_point),
                )
            })
            .sum::<Color>();

        let reflected = self.reflected_color(comps, remaining);
        let refracted = self.refracted_color(comps, remaining);

        let material = comps.object.props().material;

        if material.reflective > 0.0 && material.transparency > 0.0 {
            let reflectance = comps.schlick();
            return surface + reflected * reflectance + refracted * (1.0 - reflectance);
        }

        surface + reflected + refracted
    }

    pub fn color_at(&self, ray: Ray, remaining: u8) -> Color {
        let xs = self.intersect(ray);

        xs.hit().map_or(BLACK, |&h| {
            let comps = h.prepare_computations(ray, &xs);
            self.shade_hit(&comps, remaining)
        })
    }

    #[must_use]
    pub fn is_shadowed(&self, light: PointLight, point: Tuple) -> bool {
        let v = light.position - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = ray(point, direction);

        self.intersect(r).hit().is_some_and(|&h| h.t < distance)
    }

    pub fn reflected_color(&self, comps: &Comps, remaining: u8) -> Color {
        if comps.object.props().material.reflective == 0.0 || remaining == 0 {
            return BLACK;
        }

        let reflect_ray = ray(comps.over_point, comps.reflectv);
        let color = self.color_at(reflect_ray, remaining - 1);

        color * comps.object.props().material.reflective
    }

    pub fn refracted_color(&self, comps: &Comps, remaining: u8) -> Color {
        if comps.object.props().material.transparency == 0.0 || remaining == 0 {
            return BLACK;
        }

        // find the ratio of the first index of refraction to the second
        // inverted from the definition of Snell's Law
        let n_ratio = comps.n1 / comps.n2;

        // cos(theta_i) is the same as the dot product of the two vectors
        let cos_i = comps.eyev.dot(comps.normalv);

        // find sin(theta_t) ^ 2 via trigonometric identity
        let sin2_t = n_ratio * n_ratio * (1.0 - cos_i * cos_i);
        if sin2_t > 1.0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        // cos(theta_t) via trigonometric identity
        let cos_t = (1.0 - sin2_t).sqrt();

        // compute the direction of the refracted ray
        let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;

        // create the refracted ray
        let refract_ray = Ray::new(comps.under_point, direction);

        // find the color of the refracted ray, making sure to multiply
        // by the transparency value to account for any opacity
        self.color_at(refract_ray, remaining - 1) * comps.object.props().material.transparency
    }
}

impl Default for World {
    fn default() -> Self {
        let s1 = Sphere::default().material(
            Material::default()
                .rgb(0.8, 1.0, 0.6)
                .diffuse(0.7)
                .specular(0.2),
        );

        let s2 = Sphere::default().transform(Matrix::scaling(0.5, 0.5, 0.5));

        let light = PointLight::new(pt(-10, 10, -10), color(1, 1, 1));

        Self {
            objects: vec![s1.into(), s2.into()],
            lights: vec![light],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{pattern::test, *};

    fn twosqrttwo() -> F {
        twosqrt() / 2.0
    }

    fn twosqrt() -> F {
        F::sqrt(2.0)
    }

    #[test]
    fn creating_a_world() {
        let w = World::new();

        assert!(w.objects.is_empty());
        assert!(w.lights.is_empty());
    }

    #[test]
    fn the_default_world() {
        let light = PointLight::new(pt(-10, 10, -10), color(1, 1, 1));

        let s1 = Sphere::default().material(
            Material::default()
                .rgb(0.8, 1.0, 0.6)
                .diffuse(0.7)
                .specular(0.2),
        );

        let s2 = Sphere::default().transform(Matrix::scaling(0.5, 0.5, 0.5));

        let w = World::default();

        assert!(w.lights.contains(&light));
        assert!(w.objects.contains(&s1.into()));
        assert!(w.objects.contains(&s2.into()));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray::new(pt(0, 0, -5), v(0, 0, 1));

        let mut xs = w.intersect(r);
        assert_eq!(xs.len(), 4);
        xs.sort_unstable();
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(pt(0, 0, -5), v(0, 0, 1));
        let shape = w.objects.first().unwrap();

        let i = shape.intersection(4.0);

        let comps = i.prepare_computations(r, &[i]);
        let c = w.shade_hit(&comps, 1);

        assert_fuzzy_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let w = World {
            lights: vec![PointLight::new(pt(0, 0.25, 0), color(1, 1, 1))],
            ..Default::default()
        };

        let r = Ray::new(pt(0, 0, 0), v(0, 0, 1));
        let s = w.objects.get(1).unwrap();
        let i = s.intersection(0.5);

        let comps = i.prepare_computations(r, &[i]);
        let c = w.shade_hit(&comps, 1);
        assert_fuzzy_eq!(c, color(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray::new(pt(0, 0, -5), v(0, 1, 0));
        let c = w.color_at(r, 1);

        assert_eq!(c, color(0, 0, 0));
    }

    #[test]
    fn color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(pt(0, 0, -5), v(0, 0, 1));
        let c = w.color_at(r, 1);

        assert_fuzzy_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_an_intersection_behind_the_ray() {
        let w = {
            let mut w = World::default();

            let outer = w.objects.get_mut(0).unwrap();
            outer.props_mut().material.ambient = 1.0;

            let inner = w.objects.get_mut(1).unwrap();
            inner.props_mut().material.ambient = 1.0;
            w
        };

        let r = Ray::new(pt(0, 0, 0.75), v(0, 0, -1));
        let c = w.color_at(r, 1);

        let inner = w.objects.get(1).unwrap();

        assert_eq!(c, inner.props().material.color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        let p = pt(0, 10, 0);
        let l = w.lights[0];

        assert!(!w.is_shadowed(l, p));
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::default();
        let p = pt(10, -10, 10);
        let l = w.lights[0];

        assert!(w.is_shadowed(l, p));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();
        let p = pt(-20, 20, -20);
        let l = w.lights[0];

        assert!(!w.is_shadowed(l, p));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default();
        let p = pt(-2, 2, -2);
        let l = w.lights[0];

        assert!(!w.is_shadowed(l, p));
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let s1 = Sphere::default();
        let s2 = Sphere::default().transform(Matrix::translation(0, 0, 10));
        let light = point_light(pt(0, 0, -10), color(1, 1, 1));

        let w = World {
            objects: vec![s1.into(), s2.into()],
            lights: vec![light],
        };

        let r = ray(pt(0, 0, 5), v(0, 0, 1));
        let s = w.objects.get(1).unwrap();
        let i = s.intersection(4.0);

        let comps = i.prepare_computations(r, &[i]);
        let c = w.shade_hit(&comps, 1);
        assert_eq!(c, color(0.1, 0.1, 0.1));
    }

    #[test]
    fn reflected_color_for_a_nonreflective_material() {
        let r = ray(pt(0, 0, 0), v(0, 0, 1));

        let mut w = World::default();

        {
            let shape = w.objects.get_mut(1).unwrap().as_mut();
            shape.props_mut().material.ambient = 1.0;
        };

        let shape = w.objects.get(1).unwrap();
        let i = shape.intersection(1.0);
        let comps = i.prepare_computations(r, &[i]);
        let c = w.reflected_color(&comps, 1);

        assert_eq!(c, color(0, 0, 0));
    }

    #[test]
    fn reflected_color_for_a_reflective_material() {
        let mut w = World::default();
        let shape = Plane::default()
            .material(Material::default().reflective(0.5))
            .transform(Matrix::translation(0, -1, 0));

        w.objects.push(shape.into());

        let r = ray(pt(0, 0, -3), v(0, -F::sqrt(2.0) / 2.0, F::sqrt(2.0) / 2.0));
        let shape = w.objects.last().unwrap();

        let i = shape.intersection(F::sqrt(2.0));
        let comps = i.prepare_computations(r, &[i]);
        let c = w.reflected_color(&comps, 1);

        assert_fuzzy_eq!(c, color(0.19033, 0.23791, 0.14274));
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut w = World::default();
        let shape = Plane::default()
            .material(Material::default().reflective(0.5))
            .transform(Matrix::translation(0, -1, 0));

        w.objects.push(shape.into());

        let r = ray(pt(0, 0, -3), v(0, -F::sqrt(2.0) / 2.0, F::sqrt(2.0) / 2.0));
        let shape = w.objects.last().unwrap();

        let i = shape.intersection(F::sqrt(2.0));

        let comps = i.prepare_computations(r, &[i]);
        let c = w.shade_hit(&comps, 5);

        assert_fuzzy_eq!(c, color(0.87675, 0.92434, 0.82918));
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = World::new();

        w.lights.push(point_light(pt(0, 0, 0), color(1, 1, 1)));

        let lower = Plane::default()
            .material(Material::default().reflective(1))
            .transform(Matrix::translation(0, -1, 0));

        w.objects.push(lower.into());
        let upper = Plane::default()
            .material(Material::default().reflective(1.0))
            .transform(Matrix::translation(0, 1, 0));

        w.objects.push(upper.into());
        let r = ray(pt(0, 0, 0), v(0, 1, 0));
        let _c = w.color_at(r, 5);
    }

    #[test]
    fn the_reflected_color_at_maximum_recursive_depth() {
        let mut w = World::default();

        let shape = Plane::default()
            .material(Material::default().reflective(0.5))
            .transform(Matrix::translation(0, -1, 0));

        w.objects.push(shape.into());

        let r = ray(pt(0, 0, -3), v(0, -F::sqrt(2.0) / 2.0, F::sqrt(2.0) / 2.0));
        let shape = w.objects.last().unwrap();
        let i = shape.intersection(F::sqrt(2.0));
        let comps = i.prepare_computations(r, &[i]);
        let c = w.reflected_color(&comps, 0);

        assert_eq!(c, BLACK);
    }

    #[test]
    fn the_refracted_color_with_an_opaque_surface() {
        let w = World::default();
        let shape = w.objects.first().unwrap();
        let r = ray(pt(0, 0, -5), v(0, 0, 1));
        let xs = [shape.intersection(4.0), shape.intersection(6.0)];
        let comps = xs[0].prepare_computations(r, &xs);
        let c = w.refracted_color(&comps, 5);

        assert_eq!(c, color(0, 0, 0));
    }

    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let mut w = World::default();
        {
            let shape = w.objects.first_mut().unwrap().as_mut();
            shape.props_mut().material.transparency = 1.0;
            shape.props_mut().material.refractive_index = 1.5;
        };

        let shape = w.objects.first().unwrap();

        let r = ray(pt(0, 0, -5), v(0, 0, 1));
        let xs = [shape.intersection(4.0), shape.intersection(6.0)];
        let comps = xs[0].prepare_computations(r, &xs);

        let c = w.refracted_color(&comps, 0);

        assert_eq!(c, BLACK);
    }

    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let mut w = World::default();

        {
            let shape = w.objects.first_mut().unwrap().as_mut();
            shape.props_mut().material.transparency = 1.0;
            shape.props_mut().material.refractive_index = 1.5;
        };

        let shape = w.objects.first().unwrap();

        let r = ray(pt(0, 0, F::sqrt(2.0) / 2.0), v(0, 1, 0));
        let xs = [
            shape.intersection(-F::sqrt(2.0) / 2.0),
            shape.intersection(F::sqrt(2.0) / 2.0),
        ];

        let comps = xs[1].prepare_computations(r, &xs);
        let c = w.refracted_color(&comps, 5);

        assert_eq!(c, color(0, 0, 0));
    }

    #[test]
    fn the_refracted_color_with_a_refracted_ray() {
        let mut w = World::default();

        {
            let a = w.objects[0].as_mut();
            a.props_mut().material.ambient = 1.0;
            a.props_mut().material.pattern = Some(test());

            let b = w.objects[1].as_mut();
            b.props_mut().material.transparency = 1.0;
            b.props_mut().material.refractive_index = 1.5;
        }

        let a = w.objects[0].as_ref();
        let b = w.objects[1].as_ref();

        let r = ray(pt(0, 0, 0.1), v(0, 1, 0));
        let xs = [
            a.intersection(-0.9899),
            b.intersection(-0.4899),
            b.intersection(0.4899),
            a.intersection(0.9899),
        ];

        let comps = xs[2].prepare_computations(r, &xs);
        let c = w.refracted_color(&comps, 5);

        assert_fuzzy_eq!(c, color(0, 0.99888, 0.04721));
    }

    #[test]
    fn shade_hit_with_a_transparent_material() {
        let mut w = World::default();

        let material = Material {
            transparency: 0.5,
            refractive_index: 1.5,
            ..Default::default()
        };

        let floor = Plane::default()
            .transform(Matrix::translation(0, -1, 0))
            .material(material);

        w.objects.push(floor.into());

        let material = Material {
            color: color(1, 0, 0),
            ambient: 0.5,
            ..Default::default()
        };

        let ball = Sphere::default()
            .transform(Matrix::translation(0.0, -3.5, -0.5))
            .material(material);

        w.objects.push(ball.into());

        let r = ray(pt(0.0, 0.0, -3.0), v(0.0, -twosqrttwo(), twosqrttwo()));

        let floor = w.objects.iter().nth_back(1).unwrap();
        let xs = [floor.intersection(twosqrt())];

        let comps = xs[0].prepare_computations(r, &xs);
        let c = w.shade_hit(&comps, 5);

        assert_fuzzy_eq!(c, color(0.93642, 0.68642, 0.68642));
    }

    #[test]
    fn shade_hit_with_a_reflective_transparent_material() {
        let mut w = World::default();
        let r = ray(pt(0, 0, -3), v(0, -twosqrttwo(), twosqrttwo()));

        let material = Material {
            reflective: 0.5,
            transparency: 0.5,
            refractive_index: 1.5,
            ..Default::default()
        };

        let floor = Plane::default()
            .transform(Matrix::translation(0, -1, 0))
            .material(material);

        w.objects.push(floor.into());

        let ball = Sphere::default()
            .material(Material::default().rgb(1, 0, 0).ambient(0.5))
            .transform(Matrix::translation(0, -3.5, -0.5));

        w.objects.push(ball.into());

        let floor = w.objects.iter().nth_back(1).unwrap();
        let xs = [floor.intersection(twosqrt())];

        let comps = xs[0].prepare_computations(r, &xs);

        let c = w.shade_hit(&comps, 5);

        assert_fuzzy_eq!(c, color(0.93391, 0.69643, 0.69243));
    }
}
