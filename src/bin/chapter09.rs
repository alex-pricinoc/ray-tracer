use ray_tracer::{
    color, point_light, pt, v, view_transform, Camera, Material, Matrix, Plane, Sphere, World, PI,
};
use std::fs::File;

fn main() {
    let canvas_width = 2256;
    let canvas_height = 1504;

    let floor = Plane::default().material(Material::new().rgb(0.5, 0.45, 0.45).specular(0.0));

    let middle = Sphere::new()
        .transform(Matrix::translation(-0.5, 1, 0.5))
        .material(Material::new().rgb(0.1, 1, 0.5).diffuse(0.7).specular(0.3));

    let right = Sphere::new()
        .transform(Matrix::scaling(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5))
        .material(Material::new().rgb(0.5, 1, 0.1).diffuse(0.7).specular(0.3));

    let left = Sphere::new()
        .transform(Matrix::scaling(0.33, 0.33, 0.33).translate(-1.5, 0.33, -0.75))
        .material(Material::new().rgb(1, 0.8, 0.1).diffuse(0.7).specular(0.3));

    let world = World {
        objects: vec![floor.into(), left.into(), middle.into(), right.into()],
        lights: vec![point_light(pt(-10, 10, -10), color(1, 1, 1))],
    };

    let camera = Camera::new(canvas_width, canvas_height, PI / 3.0).transform(view_transform(
        pt(0, 1.5, -5),
        pt(0, 1, 0),
        v(0, 1, 0),
    ));

    let canvas = camera.render(&world);

    let mut file = File::create("pictures/chapter-09.ppm").unwrap();
    canvas.write_ppm(&mut file).unwrap();
}
