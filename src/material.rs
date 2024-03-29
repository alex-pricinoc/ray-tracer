use crate::{color, Color, Pattern, PointLight, Shape, Tuple, BLACK, F, WHITE};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub ambient: F,
    pub diffuse: F,
    pub specular: F,
    pub shininess: F,
    pub reflective: F,
    pub transparency: F,
    pub refractive_index: F,
    pub pattern: Option<Pattern>,
}

impl Material {
    pub fn lighting(
        &self,
        object: &dyn Shape,
        light: PointLight,
        point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color {
        #[allow(clippy::needless_late_init)]
        let ambient_light: Color;
        let diffuse_light: Color;
        let specular_light: Color;

        let color = self
            .pattern
            .map_or(self.color, |p| p.color_at_object(object, point));

        // combine the surface color with the light's color/intensity
        let effective_color = color * light.intensity;

        // find the direction to the light source
        let lightv = (light.position - point).normalize();

        // compute the ambient contribution
        ambient_light = effective_color * self.ambient;

        if in_shadow {
            return ambient_light;
        }

        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = lightv.dot(normalv);

        if light_dot_normal < 0.0 {
            diffuse_light = BLACK;
            specular_light = BLACK;
        } else {
            // compute the diffuse contribution
            diffuse_light = effective_color * self.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let reflectv = -lightv.reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);

            if reflect_dot_eye <= 0.0 {
                specular_light = BLACK;
            } else {
                // compute the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                specular_light = light.intensity * self.specular * factor;
            }
        }

        ambient_light + diffuse_light + specular_light
    }

    #[must_use]
    pub fn rgb(mut self, r: impl Into<F>, g: impl Into<F>, b: impl Into<F>) -> Self {
        self.color = color(r, g, b);

        self
    }

    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;

        self
    }

    #[must_use]
    pub fn ambient(mut self, ambient: impl Into<F>) -> Self {
        self.ambient = ambient.into();

        self
    }

    #[must_use]
    pub fn diffuse(mut self, diffuse: impl Into<F>) -> Self {
        self.diffuse = diffuse.into();

        self
    }

    #[must_use]
    pub fn specular(mut self, specular: impl Into<F>) -> Self {
        self.specular = specular.into();

        self
    }

    #[must_use]
    pub fn shininess(mut self, shininess: impl Into<F>) -> Self {
        self.shininess = shininess.into();

        self
    }

    #[must_use]
    pub fn pattern(mut self, pattern: Pattern) -> Self {
        self.pattern = Some(pattern);

        self
    }

    #[must_use]
    pub fn reflective(mut self, reflective: impl Into<F>) -> Self {
        self.reflective = reflective.into();

        self
    }

    #[must_use]
    pub fn transparency(mut self, transparency: impl Into<F>) -> Self {
        self.transparency = transparency.into();

        self
    }

    #[must_use]
    pub fn refractive_index(mut self, refractive_index: impl Into<F>) -> Self {
        self.refractive_index = refractive_index.into();

        self
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            pattern: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn the_default_material() {
        let m = Material::default();

        assert_eq!(m.color, color(1, 1, 1));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::default();
        let position = pt(0, 0, 0);

        let eyev = v(0, 0, -1);
        let normalv = v(0, 0, -1);
        let light = point_light(pt(0, 0, -10), color(1, 1, 1));
        let in_shadow = false;
        let object = Sphere::default();

        let result = m.lighting(&object, light, position, eyev, normalv, in_shadow);
        assert_fuzzy_eq!(result, color(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45_deg() {
        let m = Material::default();
        let position = pt(0, 0, 0);

        let eyev = v(0, F::sqrt(2.0) / 2.0, -F::sqrt(2.0) / 2.0);
        let normalv = v(0, 0, -1);
        let light = point_light(pt(0, 0, -10), color(1, 1, 1));
        let in_shadow = false;
        let object = Sphere::default();

        let result = m.lighting(&object, light, position, eyev, normalv, in_shadow);
        assert_fuzzy_eq!(result, color(1, 1, 1));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_deg() {
        let m = Material::default();
        let position = pt(0, 0, 0);

        let eyev = v(0, 0, -1);
        let normalv = v(0, 0, -1);
        let light = point_light(pt(0, 10, -10), color(1, 1, 1));
        let in_shadow = false;
        let object = Sphere::default();

        let result = m.lighting(&object, light, position, eyev, normalv, in_shadow);

        assert_fuzzy_eq!(result, color(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_v() {
        let m = Material::default();
        let position = pt(0, 0, 0);

        let eyev = v(0, -F::sqrt(2.0) / 2.0, -F::sqrt(2.0) / 2.0);
        let normalv = v(0, 0, -1);
        let light = point_light(pt(0, 10, -10), color(1, 1, 1));
        let in_shadow = false;
        let object = Sphere::default();

        let result = m.lighting(&object, light, position, eyev, normalv, in_shadow);

        assert_fuzzy_eq!(result, color(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::default();
        let position = pt(0, 0, 0);

        let eyev = v(0, 0, -1);
        let normalv = v(0, 0, -1);
        let light = point_light(pt(0, 0, 10), color(1, 1, 1));
        let in_shadow = false;
        let object = Sphere::default();

        let result = m.lighting(&object, light, position, eyev, normalv, in_shadow);

        assert_fuzzy_eq!(result, color(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let m = Material::default();
        let position = pt(0, 0, 0);

        let eyev = v(0, 0, -1);
        let normalv = v(0, 0, -1);
        let light = point_light(pt(0, 0, -10), color(1, 1, 1));
        let in_shadow = true;
        let object = Sphere::default();

        let result = m.lighting(&object, light, position, eyev, normalv, in_shadow);

        assert_fuzzy_eq!(result, color(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_a_pattern_applied() {
        let m = Material::default()
            .pattern(stripe(color(1, 1, 1), color(0, 0, 0)))
            .ambient(1)
            .diffuse(0)
            .specular(0);

        let eyev = v(0, 0, -1);
        let normalv = v(0, 0, -1);
        let light = point_light(pt(0, 0, -10), color(1, 1, 1));
        let object = Sphere::default();

        let c1 = m.lighting(&object, light, pt(0.9, 0, 0), eyev, normalv, false);
        let c2 = m.lighting(&object, light, pt(1.1, 0, 0), eyev, normalv, false);

        assert_fuzzy_eq!(c1, color(1, 1, 1));
        assert_fuzzy_eq!(c2, color(0, 0, 0));
    }

    #[test]
    fn reflectivity_for_the_default_material() {
        let m = Material::default();

        assert_eq!(m.reflective, 0.0);
    }

    #[test]
    fn transparency_and_refractive_index_for_the_default_material() {
        let m = Material::default();

        assert_eq!(m.transparency, 0.0);
        assert_eq!(m.refractive_index, 1.0);
    }
}
