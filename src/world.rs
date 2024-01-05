use crate::{
    color, pt, ray, Color, Comps, Intersection, Intersections, Material, Matrix, PointLight, Ray,
    Shape, Sphere, Tuple,
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

    // intersect all objects in the world with the ray, and aggregate the intersections into a single collection
    #[must_use]
    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        self.objects.iter().flat_map(|o| o.intersect(ray)).collect()
    }

    pub fn shade_hit(&self, comps: &Comps) -> Color {
        self.lights
            .iter()
            .map(|&l| {
                comps.object.props().material.lighting(
                    l,
                    comps.point,
                    comps.eyev,
                    comps.normalv,
                    self.is_shadowed(l, comps.over_point),
                )
            })
            .sum()
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        self.intersect(ray).hit().map_or(Color::black(), |&h| {
            let comps = h.prepare_computations(ray);
            self.shade_hit(&comps)
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
}

impl Default for World {
    fn default() -> Self {
        let s1 = Sphere::new().material(
            Material::new()
                .rgb(0.8, 1.0, 0.6)
                .diffuse(0.7)
                .specular(0.2),
        );

        let s2 = Sphere::new().transform(Matrix::scaling(0.5, 0.5, 0.5));

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
    use crate::*;

    #[test]
    fn creating_a_world() {
        let w = World::new();

        assert!(w.objects.is_empty());
        assert!(w.lights.is_empty());
    }

    #[test]
    fn the_default_world() {
        let light = PointLight::new(pt(-10, 10, -10), color(1, 1, 1));

        let s1 = Sphere::new().material(
            Material::new()
                .rgb(0.8, 1.0, 0.6)
                .diffuse(0.7)
                .specular(0.2),
        );

        let s2 = Sphere::new().transform(Matrix::scaling(0.5, 0.5, 0.5));

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
        let shape = w.objects.first().unwrap().as_ref();

        let i = Intersection::new(4.0, shape);

        let comps = i.prepare_computations(r);
        let c = w.shade_hit(&comps);

        assert_fuzzy_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let w = World {
            lights: vec![PointLight::new(pt(0, 0.25, 0), color(1, 1, 1))],
            ..Default::default()
        };

        let r = Ray::new(pt(0, 0, 0), v(0, 0, 1));
        let s = w.objects.get(1).unwrap().as_ref();
        let i = Intersection::new(0.5, s);

        let comps = i.prepare_computations(r);
        let c = w.shade_hit(&comps);
        assert_fuzzy_eq!(c, color(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray::new(pt(0, 0, -5), v(0, 1, 0));
        let c = w.color_at(r);

        assert_eq!(c, color(0, 0, 0));
    }

    #[test]
    fn color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(pt(0, 0, -5), v(0, 0, 1));
        let c = w.color_at(r);

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
        let c = w.color_at(r);

        let inner = w.objects.get(1).unwrap();

        assert_fuzzy_eq!(c, inner.props().material.color);
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
        let s1 = Sphere::new();
        let s2 = Sphere::new().transform(Matrix::translation(0, 0, 10));
        let light = point_light(pt(0, 0, -10), color(1, 1, 1));

        let w = World {
            objects: vec![s1.into(), s2.into()],
            lights: vec![light],
        };

        let r = ray(pt(0, 0, 5), v(0, 0, 1));
        let s = w.objects.get(1).unwrap().as_ref();
        let i = Intersection::new(4.0, s);

        let comps = i.prepare_computations(r);
        let c = w.shade_hit(&comps);
        assert_fuzzy_eq!(c, color(0.1, 0.1, 0.1));
    }
}
