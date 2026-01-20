use crate::raytracing::intersections::*;
use crate::raytracing::lighting::*;
use crate::raytracing::*;
pub mod plane;
pub mod sphere;
pub mod world;
pub use plane::*;
pub use sphere::*;
pub use world::*;

use sealed::Sealed;
mod sealed {
    pub trait Sealed {}
}

pub trait Obj: Sealed {
    fn local_intersect_from_origin(&self, r: Ray, p: Trivector) -> Vec<Intersection<'_>>;
    fn intersect(&self, r: Ray, c: &Camera) -> Vec<Intersection<'_>> {
        self.intersect_from_origin(r, c.location)
    }
    fn intersect_from_origin(&self, r: Ray, p: Trivector) -> Vec<Intersection<'_>> {
        let origin = (self.get_transform() << p).scale(self.get_scale().reciprocal());
        let r_t = ((origin
            + (self.get_transform() << r.forwards()).scale(self.get_scale().reciprocal()))
            & origin)
            .assert::<Bivector>();
        self.local_intersect_from_origin(r_t, origin)
    }
    fn local_surface_at(&self, p: Trivector) -> Vector;
    fn surface_at(&self, p: Trivector) -> Vector {
        let p = (self.get_transform() << p).scale(self.get_scale().reciprocal()) - e123;
        let mut n = self.get_transform()
            >> self
                .local_surface_at(p)
                .scale(self.get_scale())
                .scale_slope(self.get_scale().reciprocal());
        n[3] = 0.0;
        n.normalize()
    }
    fn material(&self) -> &Material;
    fn material_mut(&mut self) -> &mut Material;
    fn set_material(&mut self, m: Material);
    fn transform_t(&mut self, t: Transformation);
    fn transform(&mut self, m: Motor);
    fn get_transform(&self) -> Motor;
    fn get_scale(&self) -> Trivector;
    fn set_scale(&mut self, new_scale: Trivector);
    fn scale(&mut self, scale: Trivector);
}

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

impl Sealed for Object {}
impl Obj for Object {
    #[inline]
    fn local_intersect_from_origin(&self, r: Ray, p: Trivector) -> Vec<Intersection<'_>> {
        use Object::*;
        match self {
            Sphere(s) => s.local_intersect_from_origin(r, p),
            Plane(pl) => pl.local_intersect_from_origin(r, p),
        }
    }

    #[inline]
    fn intersect_from_origin(&self, r: Ray, p: Trivector) -> Vec<Intersection<'_>> {
        use Object::*;
        match self {
            Sphere(s) => s.intersect_from_origin(r, p),
            Plane(pl) => pl.intersect_from_origin(r, p),
        }
    }

    #[inline]
    fn local_surface_at(&self, p: Trivector) -> Vector {
        use Object::*;
        match self {
            Sphere(s) => s.local_surface_at(p),
            Plane(pl) => pl.local_surface_at(p),
        }
    }

    #[inline]
    fn surface_at(&self, p: Trivector) -> Vector {
        use Object::*;
        match self {
            Sphere(s) => s.surface_at(p),
            Plane(pl) => pl.surface_at(p),
        }
    }

    #[inline]
    fn material(&self) -> &Material {
        use Object::*;
        match self {
            Sphere(s) => &s.material,
            Plane(pl) => &pl.material,
        }
    }

    #[inline]
    fn material_mut(&mut self) -> &mut Material {
        use Object::*;
        match self {
            Sphere(s) => &mut s.material,
            Plane(pl) => &mut pl.material,
        }
    }

    #[inline]
    fn set_material(&mut self, m: Material) {
        use Object::*;
        match self {
            Sphere(s) => s.material = m,
            Plane(pl) => pl.material = m,
        };
    }

    #[inline]
    fn transform_t(&mut self, t: Transformation) {
        use Object::*;
        match self {
            Sphere(s) => s.transform_t(t),
            Plane(pl) => pl.transform_t(t),
        };
    }

    #[inline]
    fn transform(&mut self, m: Motor) {
        use Object::*;
        match self {
            Sphere(s) => s.transform(m),
            Plane(pl) => pl.transform(m),
        }
    }

    #[inline]
    fn get_transform(&self) -> Motor {
        use Object::*;
        match self {
            Sphere(s) => s.transform,
            Plane(_) => panic!("Calculating transform motor of planes not currently supported."),
        }
    }

    #[inline]
    fn get_scale(&self) -> Trivector {
        use Object::*;
        match self {
            Sphere(s) => s.scale,
            Plane(pl) => pl.scale,
        }
    }

    #[inline]
    fn set_scale(&mut self, scale: Trivector) {
        use Object::*;
        match self {
            Sphere(s) => s.set_scale(scale),
            Plane(pl) => pl.set_scale(scale),
        }
    }

    #[inline]
    fn scale(&mut self, scale: Trivector) {
        use Object::*;
        match self {
            Sphere(s) => s.scale(scale),
            Plane(pl) => pl.scale(scale),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ObjectRef<'a> {
    Sphere(&'a Sphere),
    Plane(&'a Plane),
}

impl<'a> From<&'a Object> for ObjectRef<'a> {
    fn from(o: &'a Object) -> Self {
        match o {
            Object::Sphere(s) => ObjectRef::Sphere(&s),
            Object::Plane(pl) => ObjectRef::Plane(&pl),
        }
    }
}

impl ObjectRef<'_> {
    pub fn intersect(&self, r: Ray, c: &Camera) -> Vec<Intersection<'_>> {
        use ObjectRef::*;
        match self {
            Sphere(s) => s.intersect(r, c),
            Plane(pl) => pl.intersect(r, c),
        }
    }

    pub fn surface_at(&self, p: Trivector) -> Vector {
        use ObjectRef::*;
        match self {
            Sphere(s) => s.surface_at(p),
            Plane(pl) => pl.surface_at(p),
        }
    }

    pub fn material(&self) -> &Material {
        use ObjectRef::*;
        match self {
            Sphere(s) => &s.material,
            Plane(pl) => &pl.material,
        }
    }
}

impl<'a> PartialEq for ObjectRef<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        use ObjectRef::*;
        match (self, other) {
            (Sphere(s1), Sphere(s2)) => s1 == s2,
            (Plane(pl1), Plane(pl2)) => pl1 == pl2,
            _ => false,
        }
    }
}
