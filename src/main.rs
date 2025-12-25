use pgatracer::canvas::*;
use pgatracer::pga_3::*;

struct Projectile {
    pub position: Trivector,
    pub velocity: Trivector,
}

struct Environment {
    pub gravity: Trivector,
    pub wind: Trivector,
}

impl Environment {
    fn tick(&self, proj: &Projectile) -> Projectile {
        Projectile {
            position: proj.position + proj.velocity,
            velocity: proj.velocity + self.gravity + self.wind,
        }
    }
}

fn main() {
    let env = Environment {
        gravity: Trivector::direction(0.0, -0.1, 0.0),
        wind: Trivector::direction(-0.01, 0.0, 0.0),
    };
    let mut canv = Canvas::new(900, 550);
    let red = Color::new(0.89, 0.26, 0.20);

    let mut p = Projectile {
        position: Trivector::point(0.0, 1.0, 0.0),
        velocity: Trivector::direction(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let mut ticks = 0;
    while p.position[2] < 0.0 {
        p = env.tick(&p);
        println!("Position: {:?}", p.position);
        let y = canv.height() - p.position[2].round().abs() as usize;
        let x = p.position[1].round().abs() as usize;
        let _ = canv.write_pixel(x, y, red);
        ticks += 1;
    }

    println!("Ticks until it hit the ground: {ticks}");
    canv.write_file("img.ppm").unwrap();
}
