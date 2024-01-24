use ray_tracer::{
    checkers, color, gradient, Camera, Material, Matrix, Plane, Sphere, Transforms, World, BLACK,
    WHITE,
};
use std::fs::File;

fn main() {
    let floor = Plane::default().material(Material::default().pattern(checkers(WHITE, BLACK)));

    let middle = Sphere::default()
        .transform(Matrix::translation(-0.5, 1.0, 0.5))
        .material(
            Material::default()
                .rgb(0.1, 1, 0.5)
                .diffuse(0.7)
                .specular(0.3),
        );

    let right = Sphere::default()
        .transform(Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5))
        .material(
            Material::default()
                .pattern(
                    gradient(color(0.5, 0.75, 0.1), color(0.1, 0.25, 1.0))
                        .transform(Matrix::translation(1, 0, 0) * Matrix::scaling(2, 2, 2)),
                )
                .diffuse(0.7)
                .specular(0.3),
        );

    let left = Sphere::default()
        .transform(Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33))
        .material(
            Material::default()
                .rgb(1.0, 0.8, 0.1)
                .diffuse(0.7)
                .specular(0.3),
        );

    let world = World {
        objects: vec![floor.into(), left.into(), middle.into(), right.into()],
        ..Default::default()
    };

    let canvas = Camera::default().render(&world);

    let mut file = File::create("pictures/chapter-10.ppm").unwrap();
    canvas.write_ppm(&mut file).unwrap();
}
