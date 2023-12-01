use ray_tracer::canvas::{color, Canvas};
use ray_tracer::matrix::Matrix;
use ray_tracer::tuple::point;
use ray_tracer::{F, PI};
use std::fs::File;

fn main() {
    let mut canvas = Canvas::new(600, 600);
    let center = point(canvas.width as F / 2.0, 0.0, canvas.height as F / 2.0);
    let radius = (3.0 / 8.0) * canvas.width as F;

    let twelve = point(0, 0, 1);

    for i in 1..=12 {
        let r = Matrix::rotation_y(i as F * PI / 6.0);
        let pixel = r * twelve;

        let x = (pixel.x * radius + center.x) as usize;
        let y = (pixel.z * radius + center.z) as usize;
        canvas.write_pixel(x, canvas.height - y, color(1, 1, 1))
    }

    let mut file = File::create("pictures/chapter-04.ppm").unwrap();
    canvas.write_ppm(&mut file).unwrap();
}
