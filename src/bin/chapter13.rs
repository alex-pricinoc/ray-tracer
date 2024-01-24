use ray_tracer::{
    checkers, color, Camera, Cone, Cube, Cylinder, Material, Matrix, Transforms, World, BLACK, PI,
    WHITE,
};
use std::fs::File;

fn main() {
    let room = Cube::default()
        .transform(
            Matrix::rotation_y(PI / 3.5)
                * Matrix::translation(0, 12, 0)
                * Matrix::scaling(15, 12, 15),
        )
        .material(
            Material::default()
                .pattern(checkers(WHITE, BLACK).transform(Matrix::scaling(
                    1.0 / 15.0,
                    1.0 / 12.0,
                    1.0 / 15.0,
                )))
                .specular(0),
        );

    let middle = Cylinder::default()
        .minimum(0)
        .maximum(1)
        .closed(true)
        .transform(Matrix::translation(-0.5, 0.0, 0.5))
        .material(
            Material::default()
                .color(color(0.8, 0.8, 0.4))
                .diffuse(0.3)
                .ambient(0.2)
                .specular(0.2)
                .reflective(0.9)
                .shininess(100),
        );

    let back = Cube::default()
        .transform(
            Matrix::translation(-2.7, 0.5, 3.0)
                * Matrix::scaling(0.5, 0.5, 0.5)
                * Matrix::rotation_y(PI / 5.0),
        )
        .material(
            Material::default()
                .color(color(0.1, 0.1, 0.6))
                .diffuse(0.6)
                .specular(0.4)
                .reflective(0.2)
                .shininess(200),
        );

    let right = Cone::default()
        .minimum(-3)
        .maximum(0)
        .transform(Matrix::translation(1.5, 0.75, -0.5) * Matrix::scaling(0.25, 0.25, 0.25))
        .material(
            Material::default()
                .color(color(0.5, 1.0, 0.1))
                .diffuse(0.2)
                .specular(0.2)
                .reflective(0.4)
                .shininess(200.0),
        );

    let world = World {
        objects: vec![room.into(), right.into(), middle.into(), back.into()],
        ..Default::default()
    };

    let canvas = Camera::default().render(&world);

    let mut file = File::create("pictures/chapter-13.ppm").unwrap();
    canvas.write_ppm(&mut file).unwrap();
}
