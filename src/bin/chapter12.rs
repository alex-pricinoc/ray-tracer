use ray_tracer::{
    checkers, Camera, Cube, Material, Matrix, Sphere, Transforms, World, BLACK, PI, WHITE,
};
use std::fs::File;

fn main() {
    let room = Cube::default()
        .transform(
            Matrix::rotation_y(PI / 3.5)
                * Matrix::translation(0, 12, 0)
                * Matrix::scaling(15, 12, 15),
        )
        .material(Material::default().specular(0).pattern(
            checkers(WHITE, BLACK).transform(Matrix::scaling(1.0 / 15.0, 1.0 / 12.0, 1.0 / 15.0)),
        ));

    let back = Sphere::default()
        .transform(Matrix::translation(0.7, 0.75, 3.5) * Matrix::scaling(0.75, 0.75, 0.75))
        .material(
            Material::default()
                .rgb(0.5, 0.1, 0.2)
                .diffuse(0.2)
                .specular(0.2)
                .reflective(0.9)
                .shininess(400),
        );

    let back_cube = Cube::default()
        .transform(
            Matrix::translation(-2.7, 0.5, 3.0)
                * Matrix::scaling(0.5, 0.5, 0.5)
                * Matrix::rotation_y(PI / 5.0),
        )
        .material(
            Material::default()
                .rgb(0.1, 0.1, 0.6)
                .diffuse(0.6)
                .specular(0.4)
                .reflective(0.2)
                .shininess(200),
        );

    let right = Sphere::default()
        .transform(Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5))
        .material(
            Material::default()
                .rgb(0.5, 1, 0.1)
                .diffuse(0.7)
                .specular(0.3),
        );

    let middle = Sphere::default()
        .transform(Matrix::translation(-0.5, 1.0, 0.5))
        .material(
            Material::default()
                .color(WHITE)
                .diffuse(0.3)
                .ambient(0.2)
                .specular(0.2)
                .transparency(0.3)
                .reflective(0.4)
                .shininess(300),
        );

    let left = Sphere::default()
        .transform(Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33))
        .material(
            Material::default()
                .rgb(1, 0.8, 0.1)
                .diffuse(0.7)
                .specular(0.3),
        );

    let world = World {
        objects: vec![
            room.into(),
            back.into(),
            back_cube.into(),
            right.into(),
            middle.into(),
            left.into(),
        ],
        ..Default::default()
    };

    let canvas = Camera::default().render(&world);

    let mut file = File::create("pictures/chapter-12.ppm").unwrap();
    canvas.write_ppm(&mut file).unwrap();
}
