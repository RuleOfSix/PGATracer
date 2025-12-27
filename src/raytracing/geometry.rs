pub mod sphere {
    use rand::prelude::*;

    #[derive(Debug, Copy, Clone)]
    pub enum Object {
        Sphere(Sphere),
    }

    impl PartialEq for Object {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            use Object::*;
            match (self, other) {
                (Sphere(s1), Sphere(s2)) => s1 == s2,
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Sphere {
        id: u32,
    }

    impl PartialEq for Sphere {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    impl Sphere {
        pub fn new() -> Self {
            Sphere {
                id: rand::rng().random::<u32>(),
            }
        }
    }
}
