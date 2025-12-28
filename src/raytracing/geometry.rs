pub mod sphere;
pub use sphere::*;

#[derive(Debug, Copy, Clone)]
pub enum Object<'a> {
    Sphere(&'a Sphere),
}

impl<'a> PartialEq for Object<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        use Object::*;
        match (self, other) {
            (Sphere(s1), Sphere(s2)) => s1 == s2,
        }
    }
}
