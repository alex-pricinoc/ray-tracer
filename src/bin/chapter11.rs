use ray_tracer::*;
use std::fs::File;

fn main() {
    let canvas_width = 2256;
    let canvas_height = 1504;

    let floor = glass_plane().material(
        Material::default()
            .rgb(1, 0, 0)
            .diffuse(0.7)
            .ambient(0.1)
            .specular(1)
            .shininess(300)
            .reflective(0.9)
            .transparency(0.9),
    );

    let middle = glass_sphere()
        .transform(Matrix::translation(-0.5, 1, 0.5))
        .material(
            Material::default()
                .rgb(0.1, 1, 0.5)
                .diffuse(0.7)
                .specular(0.3)
                .reflective(1),
        );

    let right = glass_sphere()
        .transform(Matrix::scaling(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5))
        .material(
            Material::default()
                .rgb(0.5, 1, 0.1)
                .diffuse(0.7)
                .specular(0.3)
                .reflective(1),
        );

    let left = Sphere::default()
        .transform(Matrix::scaling(0.33, 0.33, 0.33).translate(-1.5, 0.33, -0.75))
        .material(
            Material::default()
                .rgb(1, 0.8, 0.1)
                .diffuse(0.7)
                .specular(0.3),
        );

    let world = World {
        objects: vec![floor.into(), left.into(), middle.into(), right.into()],
        lights: vec![point_light(pt(-10, 10, -10), WHITE)],
    };

    let camera = Camera::new(canvas_width, canvas_height, PI / 3.0).transform(view_transform(
        pt(0, 1.5, -5),
        pt(0, 1, 0),
        v(0, 1, 0),
    ));

    let canvas = camera.render(&world);

    let mut file = File::create("pictures/chapter-11.ppm").unwrap();
    canvas.write_ppm(&mut file).unwrap();
}
