use crate::canvas::*;
use crate::pga_3::*;
use crate::raytracing::geometry::*;
use std::any::Any;

use sealed::Sealed;
mod sealed {
    pub trait Sealed {}
}

pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub camera: Camera,
}

pub trait WorldMember: Sealed {}
impl Sealed for Object {}
impl WorldMember for Object {}
impl Sealed for Sphere {}
impl WorldMember for Sphere {}
impl Sealed for Light {}
impl WorldMember for Light {}

impl World {
    #[inline]
    pub fn new() -> Self {
        World {
            objects: vec![],
            lights: vec![],
            camera: Camera::default(),
        }
    }

    pub fn default() -> Self {
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::new();
        s2.scale = Trivector::scale(0.5, 0.5, 0.5);

        let light = PointLight::new(Trivector::point(-10.0, 10.0, -10.0), WHITE);

        World {
            objects: vec![Object::Sphere(s1), Object::Sphere(s2)],
            lights: vec![Light::Point(light)],
            camera: Camera::new(
                e123 + 5.0 * e021,
                e021,
                -e013,
                500,
                500,
                std::f32::consts::PI / 2.0,
            ),
        }
    }

    #[allow(irrefutable_let_patterns)]
    pub fn contains<T: WorldMember + 'static>(&self, member: &T) -> bool {
        if let Some(obj) = (member as &dyn Any).downcast_ref::<Object>() {
            self.objects.contains(obj)
        } else if let Some(light) = (member as &dyn Any).downcast_ref::<Light>() {
            self.lights.contains(light)
        } else if let Some(sphere) = (member as &dyn Any).downcast_ref::<Sphere>() {
            self.objects.iter().any(|o| {
                if let Object::Sphere(s) = o {
                    s == sphere
                } else {
                    false
                }
            })
        } else {
            false
        }
    }

    #[inline]
    pub fn intersect(&self, r: Ray) -> Vec<Intersection<'_>> {
        self.objects
            .iter()
            .map(|o| o.intersect(r, &self.camera))
            .fold(
                vec![],
                |mut acc: Vec<Intersection<'_>>, xs: Vec<Intersection<'_>>| {
                    for x in xs {
                        acc.insert(acc.partition_point(|xa: &Intersection| xa.t() < x.t()), x);
                    }
                    acc
                },
            )
    }

    #[inline]
    pub fn shade_hit(&self, h: &IntersectionState<'_>) -> Color {
        let mut c = BLACK;
        for light in &self.lights {
            c = c + h
                .point()
                .lighting(h.obj().material(), &light, h.eyev(), h.surface());
        }
        c
    }

    #[inline]
    pub fn color_at(&self, r: Ray) -> Color {
        let xs = self.intersect(r);
        let Some(h) = xs.hit() else {
            return BLACK;
        };
        let i = h.precompute(&r, &self.camera);
        self.shade_hit(&i)
    }

    #[inline]
    pub fn render(&self) -> Canvas {
        let mut img = Canvas::new(self.camera.hsize, self.camera.vsize);
        for (x, y, c) in img.enumerate_mut() {
            let r = self.camera.ray_for_pixel(x, y);
            *c = self.color_at(r);
        }
        img
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn world_new_empty() {
        let w = World::new();
        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.lights.len(), 0);
    }

    #[test]
    fn default_world() {
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::new();
        s2.scale = Trivector::scale(0.5, 0.5, 0.5);

        let light = PointLight::new(Trivector::point(-10.0, 10.0, -10.0), WHITE);

        let w = World::default();
        assert_eq!(w.objects.len(), 2);
        assert_eq!(w.lights.len(), 1);
        assert!(w.contains(&Object::Sphere(s1)));
        assert!(w.contains(&Object::Sphere(s2)));
        assert!(w.contains(&Light::Point(light)));
    }

    #[test]
    fn intersect_world() {
        let w = World::default();
        let r = Ray::from((
            Trivector::point(0.0, 0.0, -5.0),
            Trivector::direction(0.0, 0.0, 1.0),
        ));

        let xs = w.intersect(r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t(), 4.0);
        assert_eq!(xs[1].t(), 4.5);
        assert_eq!(xs[2].t(), 5.5);
        assert_eq!(xs[3].t(), 6.0);
    }

    #[test]
    fn shade_intersection() {
        let w = World::default();
        let r = Ray::from((
            Trivector::point(0.0, 0.0, -5.0),
            Trivector::direction(0.0, 0.0, 1.0),
        ));
        let shape = &w.objects[0];
        let i = Intersection::new(4.0, shape.into());
        let comps = i.precompute(&r, &w.camera);
        assert_eq!(w.shade_hit(&comps), Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_ray_miss() {
        let w = World::default();
        let r = Ray::from((
            Trivector::point(0.0, 0.0, -5.0),
            Trivector::direction(0.0, 1.0, 0.0),
        ));
        let col = w.color_at(r);
        assert_eq!(col, BLACK);
    }

    #[test]
    fn color_ray_hit() {
        let w = World::default();
        let r = Ray::from((
            Trivector::point(0.0, 0.0, -5.0),
            Trivector::direction(0.0, 0.0, 1.0),
        ));
        let col = w.color_at(r);
        assert_eq!(col, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_ray_with_intersection_behind_ray() {
        let mut w = World::default();
        w.objects[0].material_mut().ambient = 1.0;
        w.objects[1].material_mut().ambient = 1.0;
        let p = Trivector::point(0.0, 0.0, 0.75);
        let r = Ray::from((p, Trivector::direction(0.0, 0.0, -1.0)));
        w.camera = Camera::new(p, -e021, -e013, 500, 500, 0.0);
        let col = w.color_at(r);
        assert_eq!(col, w.objects[1].material().color);
    }

    #[test]
    fn render_default_world() {
        use std::f32::consts::PI;
        let mut w = World::default();
        let c = Camera::new(e123 + 5.0 * e021, -e021, -e013, 11, 11, PI / 2.0);
        w.camera = c;
        let image = w.render();
        assert_eq!(
            *image.pixel_at(5, 5).unwrap(),
            Color::new(0.38066, 0.47483, 0.2855)
        );
    }
}
