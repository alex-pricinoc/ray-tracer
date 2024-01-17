use ray_tracer::{
    checkers, color, gradient, pt, v, view_transform, Camera, Material, Matrix, Plane, Sphere,
    World, BLACK, PI, WHITE,
};
use std::fs::File;

fn main() {
    let canvas_width = 2256;
    let canvas_height = 1504;

    let floor = Plane::default().material(
        Material::default()
            .pattern(checkers(WHITE, BLACK))
            .reflective(0.2),
    );

    let middle = Sphere::default()
        .transform(Matrix::translation(-0.5, 1.0, 0.5))
        .material(
            Material::default()
                .diffuse(0.2)
                .specular(0.2)
                .transparency(1)
                .refractive_index(1.04)
                .reflective(0.9)
                .shininess(600),
        );

    let right = Sphere::default()
        .transform(Matrix::scaling(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5))
        .material(
            Material::default()
                .pattern(gradient(color(0.5, 0.75, 0.1), color(0.1, 0.25, 1.0)))
                .diffuse(0.2)
                .specular(0.2)
                .reflective(1)
                .shininess(400),
        );

    let left = Sphere::default()
        .transform(Matrix::scaling(0.33, 0.33, 0.33).translate(-1.8, 0.33, 2.5))
        .material(
            Material::default()
                .rgb(1, 0.8, 0.1)
                .diffuse(0.7)
                .specular(0.3),
        );

    let world = World {
        objects: vec![floor.into(), left.into(), middle.into(), right.into()],
        ..Default::default()
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
