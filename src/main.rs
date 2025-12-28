use pgatracer::canvas::*;
use pgatracer::pga_3::*;
use pgatracer::raytracing::*;

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
    let mut canv = Canvas::new(1000, 1000);
    let col = Color::new(0.0, 0.7, 0.7);

    let camera = Camera::new(e2 + 1.0 * e0);
    let ray_dir = Trivector::direction(0.0, -1.0, 0.0);
    let mut sphere = Sphere::new();
    sphere.transform(Transformation::trans_coords(1.25, 0.0, 1.25).into());
    sphere.scale = Trivector::scale(0.4, 0.4, 0.4);

    for (x, y) in canv.enumerate().map(|t| (t.0, t.1)).collect::<Vec<_>>() {
        let ray_source = Trivector::point(x as f32 / 1000.0, 1.0, y as f32 / 1000.0);
        let ray = Ray::from((ray_source, ray_dir));
        let xs = ray.intersect(&sphere, &camera);
        if xs.hit() != None {
            canv.write_pixel(x, y, col).unwrap();
        }
    }

    canv.write_file("img.ppm").unwrap();
}
