use crate::canvas::*;
use crate::pga_3::*;
use crate::raytracing::geometry::*;
use std::any::Any;

use sealed::Sealed;
mod sealed {
    pub trait Sealed {}
}

#[derive(Debug, Clone)]
struct Sample {
    color: Color,
    section: PixelSection,
    scale: f32,
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
    pub fn intersect_from_origin(&self, r: Ray, origin: Trivector) -> Vec<Intersection<'_>> {
        self.objects
            .iter()
            .map(|o| o.intersect_from_origin(r, origin))
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
            let in_shadow = self.is_shadowed(h.over_point(), light);
            c = c + h.point().lighting(
                h.obj().material(),
                &light,
                h.eyev(),
                h.surface(),
                in_shadow,
            );
        }
        c
    }

    #[inline]
    pub fn is_shadowed(&self, point: Trivector, light: &Light) -> bool {
        #![allow(irrefutable_let_patterns)]
        let Light::Point(light) = light else {
            panic!("Non-point light shadows not implemented");
        };
        let shadow_ray = (light.position & point).normalize().assert::<Ray>();
        let xs = self.intersect_from_origin(shadow_ray, point);
        if let Some(h) = xs.hit()
            && h.t() < (light.position - point).magnitude()
        {
            return true;
        }
        false
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

    #[inline]
    pub fn render_anti_alias(&self) -> Canvas {
        let mut img = Canvas::new(self.camera.hsize, self.camera.vsize);
        for (x, y, c) in img.enumerate_mut() {
            let samples = self.render_samples_at(x as f32, y as f32, 1.0, PixelSection::Center);
            let weighted_color_sum = samples
                .iter()
                .fold(BLACK, |acc, s| acc + s.color * s.scale.powi(2) * 0.2);
            let correction_factor = samples
                .iter()
                .fold(0.0, |acc, s| acc + s.scale.powi(2) * 0.2);
            *c = weighted_color_sum / correction_factor
        }
        img
    }

    fn render_samples_at(&self, x: f32, y: f32, scale: f32, previous: PixelSection) -> Vec<Sample> {
        const COLOR_THRESHOLD: f32 = 0.01;
        const MAX_DEPTH: i32 = 4;
        let samples: Vec<Sample> = self
            .camera
            .rays_for_pixel(x, y, scale, previous)
            .iter()
            .enumerate()
            .map(|(i, r)| Sample {
                color: self.color_at(*r),
                section: PixelSection::from_index(i),
                scale: scale,
            })
            .collect();
        let mut similar_samples: Vec<Vec<&Sample>> = vec![vec![&samples[0]]];
        'outer: for sample in &samples {
            for sample_class in &mut similar_samples {
                if sample
                    .color
                    .similar_to(sample_class[0].color, COLOR_THRESHOLD)
                {
                    sample_class.push(sample);
                    continue 'outer;
                }
            }
            similar_samples.push(vec![sample]);
        }
        similar_samples.sort_unstable_by_key(|samples| samples.len());
        similar_samples
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, samples)| {
                if i == 0 || scale <= 1.0 / (2.0_f32.powi(MAX_DEPTH)) {
                    return samples
                        .into_iter()
                        .map(|s| s.clone())
                        .collect::<Vec<Sample>>();
                };
                samples
                    .iter()
                    .flat_map(|s| {
                        let new_scale = scale / 2.0;
                        let mut res = self.render_samples_at(
                            x + s.section.x() * new_scale,
                            y + s.section.y() * new_scale,
                            new_scale,
                            s.section,
                        );
                        res.push((*s).clone());
                        res
                    })
                    .collect()
            })
            .flatten()
            .collect()
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
    fn shade_intersection_in_shadow() {
        let mut w = World::default();
        w.lights[0] = Light::Point(PointLight::new(Trivector::point(0.0, 0.0, -10.0), WHITE));
        w.objects.push(Object::Sphere(Sphere::new()));
        w.objects.push(Object::Sphere(Sphere::new()));
        w.objects[3].transform_t(Transformation::trans_coords(0.0, 0.0, 10.0));

        let ray_origin = Trivector::point(0.0, 0.0, 5.0);
        w.camera.location = ray_origin;
        let r = Ray::from((ray_origin, Trivector::direction(0.0, 0.0, 1.0)));

        let shape = &w.objects[3];
        let i = Intersection::new(4.0, shape.into());
        let comps = i.precompute(&r, &w.camera);

        assert_eq!(w.shade_hit(&comps), Color::new(0.1, 0.1, 0.1));
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

    #[test]
    fn shadow_no_object_colinear() {
        let world = World::default();
        let point = Trivector::point(0.0, 10.0, 0.0);
        assert_eq!(world.is_shadowed(point, &world.lights[0]), false);
    }

    #[test]
    fn shadow_object_occlude_light() {
        let world = World::default();
        let point = Trivector::point(10.0, -10.0, 10.0);
        assert_eq!(world.is_shadowed(point, &world.lights[0]), true);
    }

    #[test]
    fn shadow_object_behind_light() {
        let world = World::default();
        let point = Trivector::point(-20.0, 20.0, -20.0);
        assert_eq!(world.is_shadowed(point, &world.lights[0]), false);
    }

    #[test]
    fn shadow_object_behind_point() {
        let world = World::default();
        let point = Trivector::point(-2.0, 2.0, -2.0);
        assert_eq!(world.is_shadowed(point, &world.lights[0]), false);
    }
}
