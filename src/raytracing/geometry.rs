use crate::raytracing::intersections::*;
use crate::raytracing::lighting::*;
use crate::raytracing::*;
pub mod sphere;
pub mod world;
pub use sphere::*;
pub use world::*;

use sealed::Sealed;
mod sealed {
    pub trait Sealed {}
}

pub trait Obj: Sealed {
    fn intersect(&self, r: Ray, c: &Camera) -> Vec<Intersection<'_>>;
    fn surface_at(&self, p: Trivector) -> Vector;
    fn material(&self) -> &Material;
    fn material_mut(&mut self) -> &mut Material;
    fn set_material(&mut self, m: Material);
    fn transform_t(&mut self, t: Transformation);
    fn transform(&mut self, m: Motor);
}

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Sphere(Sphere),
}

impl Sealed for Object {}
impl Obj for Object {
    fn intersect(&self, r: Ray, c: &Camera) -> Vec<Intersection<'_>> {
        use Object::*;
        match self {
            Sphere(s) => s.intersect(r, c),
        }
    }

    fn surface_at(&self, p: Trivector) -> Vector {
        use Object::*;
        match self {
            Sphere(s) => s.surface_at(p),
        }
    }

    fn material(&self) -> &Material {
        use Object::*;
        match self {
            Sphere(s) => &s.material,
        }
    }

    fn material_mut(&mut self) -> &mut Material {
        use Object::*;
        match self {
            Sphere(s) => &mut s.material,
        }
    }

    fn set_material(&mut self, m: Material) {
        use Object::*;
        match self {
            Sphere(s) => s.material = m,
        };
    }

    fn transform_t(&mut self, t: Transformation) {
        use Object::*;
        match self {
            Sphere(s) => s.transform_t(t),
        };
    }

    fn transform(&mut self, m: Motor) {
        use Object::*;
        match self {
            Sphere(s) => s.transform(m),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ObjectRef<'a> {
    Sphere(&'a Sphere),
}

impl<'a> From<&'a Object> for ObjectRef<'a> {
    fn from(o: &'a Object) -> Self {
        match o {
            Object::Sphere(s) => ObjectRef::Sphere(&s),
        }
    }
}

impl ObjectRef<'_> {
    pub fn intersect(&self, r: Ray, c: &Camera) -> Vec<Intersection<'_>> {
        use ObjectRef::*;
        match self {
            Sphere(s) => s.intersect(r, c),
        }
    }

    pub fn surface_at(&self, p: Trivector) -> Vector {
        use ObjectRef::*;
        match self {
            Sphere(s) => s.surface_at(p),
        }
    }

    pub fn material(&self) -> &Material {
        use ObjectRef::*;
        match self {
            Sphere(s) => &s.material,
        }
    }
}

impl<'a> PartialEq for ObjectRef<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        use ObjectRef::*;
        match (self, other) {
            (Sphere(s1), Sphere(s2)) => s1 == s2,
        }
    }
}
