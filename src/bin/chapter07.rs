use ray_tracer::{Camera, Material, Matrix, Shape, Sphere, World, PI};
use std::fs::File;

fn main() {
    let floor = Sphere::default()
        .transform(Matrix::scaling(10, 0.01, 10))
        .material(Material::default().rgb(1, 0.9, 0.9).specular(0.0));

    let left_wall = Sphere::default()
        .transform(
            Matrix::scaling(10, 0.01, 10)
                .rotate_x(PI / 2.0)
                .rotate_y(-PI / 4.0)
                .translate(0, 0, 5),
        )
        .material(floor.props().material);

    let right_wall = Sphere::default()
        .transform(
            Matrix::scaling(10, 0.01, 10)
                .rotate_x(PI / 2.0)
                .rotate_y(PI / 4.0)
                .translate(0, 0, 5),
        )
        .material(floor.props().material);

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
        objects: vec![
            floor.into(),
            left_wall.into(),
            right_wall.into(),
            left.into(),
            middle.into(),
            right.into(),
        ],
        ..Default::default()
    };

    let canvas = Camera::default().render(&world);

    let mut file = File::create("pictures/chapter-07.ppm").unwrap();
    canvas.write_ppm(&mut file).unwrap();
}
