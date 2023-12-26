use ray_tracer::{
    color, pt, v, view_transform, Camera, Material, Matrix, PointLight, Shape, Sphere, World, PI,
};
use std::fs::File;

fn main() {
    let canvas_width = 2256;
    let canvas_height = 1504;

    let floor = Sphere::new()
        .transform(Matrix::scaling(10, 0.01, 10))
        .material(Material::new().rgb(1, 0.9, 0.9).specular(0.0));

    let left_wall = Sphere::new()
        .transform(
            Matrix::scaling(10, 0.01, 10)
                .rotate_x(PI / 2.0)
                .rotate_y(-PI / 4.0)
                .translate(0, 0, 5),
        )
        .material(floor.props().material);

    let right_wall = Sphere::new()
        .transform(
            Matrix::scaling(10, 0.01, 10)
                .rotate_x(PI / 2.0)
                .rotate_y(PI / 4.0)
                .translate(0, 0, 5),
        )
        .material(floor.props().material);

    let middle = Sphere::new()
        .transform(Matrix::translation(-0.5, 1, 0.5))
        .material(Material::new().rgb(0.1, 1, 0.5).diffuse(0.7).specular(0.3));

    let right = Sphere::new()
        .transform(Matrix::scaling(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5))
        .material(Material::new().rgb(0.5, 1, 0.1).diffuse(0.7).specular(0.3));

    let left = Sphere::new()
        .transform(Matrix::scaling(0.33, 0.33, 0.33).translate(-1.5, 0.33, -0.75))
        .material(Material::new().rgb(1, 0.8, 0.1).diffuse(0.7).specular(0.3));

    let world = World::from((
        vec![floor, left_wall, right_wall, left, middle, right],
        vec![PointLight::new(pt(-10, 10, -10), color(1, 1, 1))],
    ));

    let camera = Camera::new(canvas_width, canvas_height, PI / 3.0).transform(view_transform(
        pt(0, 1.5, -5),
        pt(0, 1, 0),
        v(0, 1, 0),
    ));

    let canvas = camera.render(&world);

    let mut file = File::create("pictures/chapter-07.ppm").unwrap();
    canvas.write_ppm(&mut file).unwrap();
}