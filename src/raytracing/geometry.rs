use crate::raytracing::*;
pub mod sphere;
pub mod world;
pub use sphere::*;
pub use world::*;

use sealed::Sealed;
mod sealed {
    pub trait Sealed {}
}

#[derive(Debug, PartialEq)]
pub enum Object {
    Sphere(Sphere),
}

pub trait Obj: WorldMember + Sealed {
    fn intersect(&self, r: Ray, c: &Camera) -> Vec<Intersection<'_>>;
}

impl Sealed for Object {}
impl Obj for Object {
    fn intersect(&self, r: Ray, c: &Camera) -> Vec<Intersection<'_>> {
        use Object::*;
        match self {
            Sphere(s) => s.intersect(r, c),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ObjectRef<'a> {
    Sphere(&'a Sphere),
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
