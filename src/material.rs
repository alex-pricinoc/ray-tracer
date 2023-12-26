use crate::{color, Color, PointLight, Tuple, F};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub ambient: F,
    pub diffuse: F,
    pub specular: F,
    pub shininess: F,
}

impl Material {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lighting(
        &self,
        light: PointLight,
        position: Tuple,
        eyev: Tuple,
        normalv: Tuple,
    ) -> Color {
        #[allow(clippy::needless_late_init)]
        let ambient_light: Color;
        let diffuse_light: Color;
        let specular_light: Color;

        // combine the surface color with the light's color/intensity
        let effective_color = self.color * light.intensity;

        // find the direction to the light source
        let lightv = (light.position - position).normalize();

        // compute the ambient contribution
        ambient_light = effective_color * self.ambient;
        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = lightv.dot(normalv);

        if light_dot_normal < 0.0 {
            diffuse_light = Color::black();
            specular_light = Color::black();
        } else {
            // compute the diffuse contribution
            diffuse_light = effective_color * self.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let reflectv = -lightv.reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);

            if reflect_dot_eye <= 0.0 {
                specular_light = Color::black();
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
    pub fn ambient(mut self, ambient: F) -> Self {
        self.ambient = ambient;
        self
    }

    #[must_use]
    pub fn diffuse(mut self, diffuse: F) -> Self {
        self.diffuse = diffuse;
        self
    }

    #[must_use]
    pub fn specular(mut self, specular: F) -> Self {
        self.specular = specular;
        self
    }

    #[must_use]
    pub fn shininess(mut self, shininess: F) -> Self {
        self.shininess = shininess;
        self
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: color(1, 1, 1),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
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
        let m = Material::new();
        let position = pt(0, 0, 0);

        let eyev = v(0, 0, -1);
        let normalv = v(0, 0, -1);
        let light = PointLight::new(pt(0, 0, -10), color(1, 1, 1));

        let result = m.lighting(light, position, eyev, normalv);
        assert_fuzzy_eq!(result, color(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45_deg() {
        let m = Material::new();
        let position = pt(0, 0, 0);

        let eyev = v(0, F::sqrt(2.0) / 2.0, -F::sqrt(2.0) / 2.0);
        let normalv = v(0, 0, -1);
        let light = PointLight::new(pt(0, 0, -10), color(1, 1, 1));

        let result = m.lighting(light, position, eyev, normalv);
        assert_fuzzy_eq!(result, color(1, 1, 1));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_deg() {
        let m = Material::new();
        let position = pt(0, 0, 0);

        let eyev = v(0, 0, -1);
        let normalv = v(0, 0, -1);
        let light = PointLight::new(pt(0, 10, -10), color(1, 1, 1));

        let result = m.lighting(light, position, eyev, normalv);

        assert_fuzzy_eq!(result, color(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_v() {
        let m = Material::new();
        let position = pt(0, 0, 0);

        let eyev = v(0, -F::sqrt(2.0) / 2.0, -F::sqrt(2.0) / 2.0);
        let normalv = v(0, 0, -1);
        let light = PointLight::new(pt(0, 10, -10), color(1, 1, 1));

        let result = m.lighting(light, position, eyev, normalv);

        assert_fuzzy_eq!(result, color(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::new();
        let position = pt(0, 0, 0);

        let eyev = v(0, 0, -1);
        let normalv = v(0, 0, -1);
        let light = PointLight::new(pt(0, 0, 10), color(1, 1, 1));

        let result = m.lighting(light, position, eyev, normalv);

        assert_fuzzy_eq!(result, color(0.1, 0.1, 0.1));
    }
}
