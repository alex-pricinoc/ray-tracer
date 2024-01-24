use ray_tracer::{Camera, Material, Matrix, Plane, Sphere, Transforms, World};
use std::fs::File;

fn main() {
    let floor = Plane::default().material(Material::default().rgb(0.5, 0.45, 0.45).specular(0.0));

    let middle = Sphere::default()
        .transform(Matrix::translation(-0.5, 1, 0.5))
        .material(
            Material::default()
                .rgb(0.1, 1, 0.5)
                .diffuse(0.7)
                .specular(0.3),
        );

    let right = Sphere::default()
        .transform(Matrix::scaling(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5))
        .material(
            Material::default()
                .rgb(0.5, 1, 0.1)
                .diffuse(0.7)
                .specular(0.3),
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
        ..Default::default()
    };
    let canvas = Camera::default().render(&world);

    let mut file = File::create("pictures/chapter-09.ppm").unwrap();
    canvas.write_ppm(&mut file).unwrap();
}
