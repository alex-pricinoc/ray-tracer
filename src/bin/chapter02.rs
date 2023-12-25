use log::*;
use ray_tracer::{color, point, vector, Canvas, Tuple};
use std::fs::File;

#[derive(Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

#[derive(Debug)]
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(projectile: &mut Projectile, env: &Environment) {
    projectile.position = projectile.position + projectile.velocity;
    projectile.velocity = projectile.velocity + env.gravity + env.wind;
}

fn main() {
    env_logger::init();

    let mut canvas = Canvas::new_with_color(1350, 825, color(1, 1, 1));

    let mut projectile = Projectile {
        position: point(0, 0, 0),
        velocity: vector(1, 1.8, 0).normalize() * 11.25,
    };

    let environment = Environment {
        gravity: vector(0, -0.1, 0),
        wind: vector(0.01, 0, 0),
    };

    let color = color(1, 0, 0);

    while projectile.position.y >= 0.0 {
        let x = projectile.position.x.round() as usize;
        let y = projectile.position.y.round() as usize;

        canvas.write_pixel(x, canvas.height - 1 - y, color);

        tick(&mut projectile, &environment);

        trace!("projectile is at x: {x:>4} y: {y:>4}");
    }

    let mut file = File::create("pictures/chapter-02.ppm").unwrap();
    canvas.write_ppm(&mut file).unwrap();
}
