use pgatracer::canvas::*;
use pgatracer::pga_3::*;
use pgatracer::raytracing::lighting::*;
use pgatracer::raytracing::*;

fn main() {
    use std::f32::consts::PI;
    let mut canv = Canvas::new(1000, 1000);

    let camera = Camera::new(e2 + 1.0 * e0);
    let ray_dir = Trivector::direction(0.0, -1.0, 0.0);
    let mut sphere = Sphere::new();
    sphere.material.color = Color::new(0.5, 0.0, 0.5);
    sphere.scale = Trivector::scale(0.5, 0.1, 1.0);
    sphere.transform(Transformation::rotation(e31, PI / 4.0).into());
    sphere.transform(Transformation::rotation(e23, PI / 6.0).into());

    let light_pos = Trivector::point(-0.25, 2.0, -0.25);
    let light_color = WHITE;
    let light = PointLight::new(light_pos, light_color);

    for (x, y) in canv.enumerate().map(|t| (t.0, t.1)).collect::<Vec<_>>() {
        let ray_source =
            Trivector::point((x as f32 - 500.0) / 600.0, 1.0, (y as f32 - 500.0) / 600.0);
        let ray = Ray::from((ray_source, ray_dir));
        let xs = sphere.intersect(ray, &camera);
        if let Some(h) = xs.hit() {
            let point = ray.position(h.t, &camera);
            let surface = sphere.surface_at(point);
            let ObjectRef::Sphere(s) = h.obj;
            let eye = -ray.forwards();
            let color = point.lighting(&s.material, &light, eye, surface);
            canv.write_pixel(x, y, color).unwrap();
        }
    }

    canv.write_file("img.ppm").unwrap();
}
