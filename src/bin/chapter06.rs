use ray_tracer::{color, point_light, pt, Canvas, Intersections, Material, Ray, Shape, Sphere, F};
use std::fs::File;

fn main() {
    let ray_origin = pt(0, 0, -5);
    let wall_z = 10;
    let wall_size = 7.0;
    let canvas_pixels = 2256;
    let half = wall_size / 2.0;
    let pixel_size = wall_size / canvas_pixels as F;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    let shape = Sphere::default().material(Material::default().rgb(1, 0.2, 1));

    let light = point_light(pt(-10, 10, -10), color(1, 1, 1));

    let cross = (0..canvas_pixels).flat_map(|y| (0..canvas_pixels).map(move |x| (x, y)));

    let object = Sphere::default();

    for (x, y) in cross {
        // compute the world x coordinate (left = -half, right = half)
        let world_x = -half + pixel_size * x as F;
        // compute the world y coordinate (top = +half, bottom = -half)
        let world_y = half - pixel_size * y as F;

        // describe the point on the wall that the ray will target
        let position = pt(world_x, world_y, wall_z);

        let r = Ray::new(ray_origin, (position - ray_origin).normalize());

        if let Some(hit) = shape.intersect(r).hit() {
            let point = r.position(hit.t);
            let normal = hit.object.normal_at(point);
            let eye = -r.direction;

            let color = hit
                .object
                .props()
                .material
                .lighting(&object, light, point, eye, normal, false);

            canvas.write_pixel(x, y, color);
        }
    }

    let mut file = File::create("pictures/chapter-06.ppm").unwrap();
    canvas.write_ppm(&mut file).unwrap();
}
