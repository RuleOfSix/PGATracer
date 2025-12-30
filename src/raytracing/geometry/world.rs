use crate::canvas::{Color, WHITE};
use crate::pga_3::*;
use crate::raytracing::geometry::*;
use crate::raytracing::lighting::*;
use std::any::Any;

use sealed::Sealed;
mod sealed {
    pub trait Sealed {}
}

pub struct World {
    objects: Vec<Object>,
    lights: Vec<Light>,
}

pub trait WorldMember: Sealed {}
impl Sealed for Object {}
impl WorldMember for Object {}
impl Sealed for Sphere {}
impl WorldMember for Sphere {}
impl Sealed for Light {}
impl WorldMember for Light {}
impl Sealed for World {}
impl WorldMember for World {}

impl super::Sealed for World {}
impl Obj for World {
    fn intersect(&self, r: Ray, c: &Camera) -> Vec<Intersection<'_>> {
        self.objects
            .iter()
            .map(|o| o.intersect(r, c))
            .fold(vec![], |mut acc, xs| {
                for x in xs {
                    acc.insert(acc.partition_point(|xa: &Intersection| xa.t < x.t), x);
                }
                acc
            })
    }
}

impl World {
    pub const fn new() -> Self {
        World {
            objects: vec![],
            lights: vec![],
        }
    }

    pub const fn objects(&self) -> &Vec<Object> {
        &self.objects
    }

    pub const fn lights(&self) -> &Vec<Light> {
        &self.lights
    }

    pub fn default_world() -> Self {
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::new();
        s2.scale = Trivector::scale(0.5, 0.5, 0.5);

        let light = PointLight::new(Trivector::point(-10.0, -10.0, -10.0), WHITE);

        World {
            objects: vec![Object::Sphere(s1), Object::Sphere(s2)],
            lights: vec![Light::Point(light)],
        }
    }

    #[allow(irrefutable_let_patterns)]
    pub fn contains<T: WorldMember + 'static>(&self, member: &T) -> bool {
        if let Some(obj) = (member as &dyn Any).downcast_ref::<Object>() {
            self.objects().contains(obj)
        } else if let Some(light) = (member as &dyn Any).downcast_ref::<Light>() {
            self.lights().contains(light)
        } else if let Some(sphere) = (member as &dyn Any).downcast_ref::<Sphere>() {
            self.objects().iter().any(|o| {
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn world_new_empty() {
        let w = World::new();
        assert_eq!(w.objects().len(), 0);
        assert_eq!(w.lights().len(), 0);
    }

    #[test]
    fn default_world() {
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::new();
        s2.scale = Trivector::scale(0.5, 0.5, 0.5);

        let light = PointLight::new(Trivector::point(-10.0, -10.0, -10.0), WHITE);

        let w = World::default_world();
        assert_eq!(w.objects().len(), 2);
        assert_eq!(w.lights().len(), 1);
        assert!(w.contains(&Object::Sphere(s1)));
        assert!(w.contains(&Object::Sphere(s2)));
        assert!(w.contains(&Light::Point(light)));
    }

    #[test]
    fn intersect_world() {
        let w = World::default_world();
        let r = Ray::from((
            Trivector::point(0.0, 0.0, -5.0),
            Trivector::direction(0.0, 0.0, 1.0),
        ));
        let c = Camera::new(e3 - 5.0 * e0);

        let xs = w.intersect(r, &c);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }
}
