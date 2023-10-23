use ray_tracer::tuple::*;

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
    let mut projectile = Projectile::new(
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(1.0, 1.0, 0.0).normalize(),
    );

    let environment = Environment::new(
        Tuple::vector(0.0, -0.1, 0.0),
        Tuple::vector(-0.01, 0.0, 0.0),
    );

    let mut iteration = 0;

    while projectile.position.y > 0.0 {
        println!("{}: {:?}", iteration, projectile);

        projectile.tick(&environment);
        iteration += 1;
    }

    println!("FINISHED => {}: {:?}", iteration, projectile);
}
