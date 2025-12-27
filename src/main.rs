use pgatracer::canvas::*;
use pgatracer::pga_3::*;

/*
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
*/

fn main() {
    use std::f32::consts::PI;

    let mut canv = Canvas::new(900, 900);
    let white = Color::new(1.0, 1.0, 1.0);

    let z_axis = Bivector::from([1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    let canvas_correction = Motor::from(Transformation::translation(Trivector::direction(
        450.0, 450.0, 0.0,
    )));
    let twelve = canvas_correction >> Trivector::point(0.0, 400.0, 0.0);
    let hour_turn = Motor::from(Transformation::rotation(
        canvas_correction >> z_axis,
        PI / 6.0,
    ));

    let mut hour = twelve;
    for _ in 0..12 {
        canv.write_pixel(-hour[1] as usize, -hour[2] as usize, white)
            .unwrap();
        hour = hour_turn >> hour;
    }
    canv.write_file("img.ppm").unwrap();
}
