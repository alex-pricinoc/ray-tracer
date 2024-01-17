use crate::{pt, v, view_transform, Canvas, Matrix, Ray, World, F, PI, REFLECTION_DEPTH};
use itertools::iproduct;
use rayon::prelude::*;

pub struct Camera {
    hsize: usize,
    vsize: usize,
    #[allow(dead_code)]
    field_of_view: F,
    half_width: F,
    half_height: F,
    pixel_size: F,
    transform: Matrix<4>,
}
impl Default for Camera {
    fn default() -> Self {
        Self::new(2256, 1504, PI / 3.0).transform(view_transform(
            pt(0, 1.5, -5),
            pt(0, 1, 0),
            v(0, 1, 0),
        ))
    }
}

impl Camera {
    #[must_use]
    #[allow(clippy::similar_names)]
    pub fn new(hsize: usize, vsize: usize, field_of_view: F) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as F / vsize as F;

        let half_width;
        let half_height;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        let pixel_size = half_width * 2.0 / hsize as F;

        Self {
            hsize,
            vsize,
            field_of_view,
            half_width,
            half_height,
            pixel_size,
            transform: Matrix::identity(),
        }
    }

    #[must_use]
    pub fn transform(mut self, transform: Matrix<4>) -> Self {
        self.transform = transform;

        self
    }

    fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        // the offset from the edge of the canvas to the pixel's center
        let offset_x = (0.5 + x as F) * self.pixel_size;
        let offset_y = (0.5 + y as F) * self.pixel_size;

        // the untransformed coordinates of the pixel in world space.
        // (remember that the camera looks toward -z, so +x is to the *left*.)
        let world_x = self.half_width - offset_x;
        let world_y = self.half_height - offset_y;

        // using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction vector.
        // (remember that the canvas is at z=-1)
        let wall_point = self.transform.inverse() * pt(world_x, world_y, -1);
        let origin = self.transform.inverse() * pt(0, 0, 0);

        let direction = (wall_point - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut canvas = Canvas::new(self.hsize, self.vsize);

        let pixels = iproduct!(0..canvas.width, 0..canvas.height)
            .par_bridge()
            .map(|(x, y)| {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray, REFLECTION_DEPTH);

                (x, y, color)
            })
            .collect::<Vec<_>>();

        for (x, y, color) in pixels {
            canvas.write_pixel(x, y, color);
        }

        canvas
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;

        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, PI / 2.0);
        assert_eq!(c.transform, Matrix::identity());
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_camera() {
        let c = Camera::new(200, 125, PI / 2.0);

        assert_fuzzy_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_camera() {
        let c = Camera::new(125, 200, PI / 2.0);

        assert_fuzzy_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);

        assert_fuzzy_eq!(r.origin, pt(0, 0, 0));
        assert_fuzzy_eq!(r.direction, v(0, 0, -1));
    }

    #[test]
    fn construction_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);

        assert_fuzzy_eq!(r.origin, pt(0, 0, 0));
        assert_fuzzy_eq!(r.direction, v(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructin_a_ray_when_the_camera_is_transformed() {
        let c = Camera::new(201, 101, PI / 2.0)
            .transform(Matrix::rotation_y(PI / 4.0) * Matrix::translation(0, -2, 5));

        let r = c.ray_for_pixel(100, 50);

        assert_fuzzy_eq!(r.origin, pt(0, 2, -5));
        assert_fuzzy_eq!(r.direction, v(F::sqrt(2.0) / 2.0, 0, -F::sqrt(2.0) / 2.0));
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = World::default();
        let from = pt(0, 0, -5);
        let to = pt(0, 0, 0);
        let up = v(0, 1, 0);

        let c = Camera::new(11, 11, PI / 2.0).transform(view_transform(from, to, up));

        let image = c.render(&w);

        assert_fuzzy_eq!(image.pixel_at(5, 5), color(0.38066, 0.47583, 0.2855));
    }
}
