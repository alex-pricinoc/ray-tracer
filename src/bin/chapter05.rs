use ray_tracer::{color, pt, Canvas, Intersections, Matrix, Ray, Shape, Sphere, PI};
use std::fs::File;

fn main() {
    env_logger::init();

    let ray_origin = pt(0, 0, -5);
    let wall_z = 10;
    let canvas_pixels = 1000;

    let wall_size = 7.0;
    let half = wall_size / 2.0;
    let pixel_size = wall_size / canvas_pixels as f64;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let shape = Sphere::new()
        // shrink it, and rotate it!
        .transform(Matrix::scaling(0.5, 1, 1).rotate_z(PI / 4.0));

    for y in 0..canvas_pixels {
        // compute the world y coordinate (top = +half, bottom = -half)
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels {
            // compute the world x coordinate (left = -half, right = half)
            let world_x = -half + pixel_size * x as f64;

            // describe the point on the wall that the ray will target
            let position = pt(world_x, world_y, wall_z);

            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(r);

            if xs.hit().is_some() {
                canvas.write_pixel(x, y, color(1, 0, 0));
            }
        }
    }

    let mut file = File::create("pictures/chapter-05.ppm").unwrap();
    canvas.write_ppm(&mut file).unwrap();
}
