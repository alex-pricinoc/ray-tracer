use crate::{
    color, pt, Color, Comps, Intersection, Intersections, Material, Matrix, PointLight, Ray, Shape,
    Sphere,
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

    pub fn shade_hit(&self, comps: &Comps) -> Color {
        self.lights
            .iter()
            .map(|&l| {
                comps
                    .object
                    .props()
                    .material
                    .lighting(l, comps.point, comps.eyev, comps.normalv)
            })
            .sum()
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        if let Some(hit) = self.intersect(ray).hit() {
            let comps = hit.prepare_computations(ray);
            self.shade_hit(&comps)
        } else {
            Color::black()
        }
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

impl<S: Into<Box<dyn Shape>>> From<(Vec<S>, Vec<PointLight>)> for World {
    fn from((objects, lights): (Vec<S>, Vec<PointLight>)) -> Self {
        Self {
            objects: objects.into_iter().map(Into::into).collect(),
            lights,
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
}
