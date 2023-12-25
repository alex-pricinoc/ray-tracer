use log::*;
use ray_tracer::{point, vector, Tuple};

#[derive(Debug)]
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

#[derive(Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

impl Projectile {
    fn new(position: Tuple, velocity: Tuple) -> Self {
        Self { position, velocity }
    }

    fn tick(&mut self, env: &Environment) {
        self.position = self.position + self.velocity;
        self.velocity = self.velocity + env.gravity + env.wind;
    }
}

impl Environment {
    fn new(gravity: Tuple, wind: Tuple) -> Self {
        Self { gravity, wind }
    }
}

fn main() {
    env_logger::init();

    let mut projectile = Projectile::new(point(0.0, 1.0, 0.0), vector(1.0, 1.0, 0.0).normalize());

    let environment = Environment::new(vector(0.0, -0.1, 0.0), vector(-0.01, 0.0, 0.0));

    let mut iteration = 0;

    while projectile.position.y > 0.0 {
        trace!("{iteration}: {projectile:?}");

        projectile.tick(&environment);
        iteration += 1;
    }

    println!("Done: iteration {iteration}, {projectile:?}");
}
