use crate::{Color, Tuple};

#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let position = point(0, 0, 0);
        let intensity = color(1, 1, 1);

        let light = PointLight::new(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
