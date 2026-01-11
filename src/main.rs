use pgatracer::canvas::*;
use pgatracer::pga_3::*;
use pgatracer::raytracing::lighting::*;
use pgatracer::raytracing::*;

fn main() {
    use std::f32::consts::PI;
    let cam_loc = Trivector::point(0.0, 1.5, -5.0);
    let cam_target = Trivector::point(0.0, 1.0, 0.0);

    let camera = Camera::new(
        cam_loc,
        (cam_target - cam_loc).normalize(),
        -e013,
        2000,
        1000,
        PI / 3.0,
    );

    let mut room_material = Material::new();
    room_material.color = Color::new(1.0, 0.9, 0.9);
    room_material.specular = 0.0;

    let mut floor_s = Sphere::new();
    floor_s.scale = Trivector::scale(10.0, 0.01, 10.0);
    floor_s.material = room_material.clone();
    let floor = Object::Sphere(floor_s);

    let mut left_wall = Sphere::new();
    left_wall.scale = Trivector::scale(10.0, 0.01, 10.0);
    left_wall.material = room_material.clone();
    left_wall.transform_t(Transformation::rotation(e12, PI / 2.0));
    left_wall.transform_t(Transformation::rotation(e31, PI / -4.0));
    left_wall.transform_t(Transformation::trans_coords(0.0, 0.0, 5.0));
    left_wall.normalize();
    let left_wall = Object::Sphere(left_wall);

    let mut right_wall = Sphere::new();
    right_wall.scale = Trivector::scale(10.0, 0.01, 10.0);
    right_wall.material = room_material.clone();
    right_wall.transform_t(Transformation::rotation(e12, PI / 2.0));
    right_wall.transform_t(Transformation::rotation(e31, PI / 4.0));
    right_wall.transform_t(Transformation::trans_coords(0.0, 0.0, 5.0));
    right_wall.normalize();
    let right_wall = Object::Sphere(right_wall);

    let mut middle = Sphere::new();
    middle.transform_t(Transformation::trans_coords(-0.5, 1.0, 0.5));
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    let middle = Object::Sphere(middle);

    let mut right = Sphere::new();
    right.transform_t(Transformation::trans_coords(1.5, 0.5, -0.5));
    right.scale = Trivector::scale(0.5, 0.5, 0.5);
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    let right = Object::Sphere(right);

    let mut left = Sphere::new();
    left.transform_t(Transformation::trans_coords(-1.5, 0.33, -0.75));
    left.scale = Trivector::scale(0.33, 0.33, 0.33);
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    let left = Object::Sphere(left);

    let light1_loc = Trivector::point(-10.0, 10.0, -10.0);
    let light = Light::Point(PointLight::new(light1_loc, Color::new(0.1, 0.5, 0.1)));

    let light2_loc = Motor::from(Transformation::rotation(e31, PI / 2.2)) >> light1_loc;
    let light2 = Light::Point(PointLight::new(light2_loc, Color::new(0.1, 0.1, 0.5)));

    let mut world = World::new();
    world.camera = camera;
    world.objects = vec![floor, left_wall, right_wall, left, middle, right];
    world.lights = vec![light, light2];

    world.render().write_file("img.ppm").unwrap();
}
