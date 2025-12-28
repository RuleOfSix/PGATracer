use crate::pga_3::*;
use rand::prelude::*;

#[derive(Debug)]
pub struct Sphere {
    id: u32,
    pub transform: Motor,
    pub scale: Trivector,
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
            transform: Motor::from([1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
            scale: Trivector::from([1.0, 1.0, 1.0, 1.0]),
        }
    }

    pub fn transform(&mut self, m: Motor) {
        self.transform = match m * self.transform {
            Versor::Even(m) => m,
            Versor::KVec(AnyKVector::Zero(s)) => Motor::from(s),
            Versor::KVec(AnyKVector::Two(bv)) => Motor::from(bv),
            Versor::KVec(AnyKVector::Four(ps)) => Motor::from(ps),
            _ => panic!("motor * motor should = motor"),
        };
    }

    #[inline]
    pub fn surface_at(&self, p: Trivector) -> Vector {
        let p = ((self.transform << p) - e123).scale(self.scale.reciprocal());
        let s = (self.transform >> p).scale(self.scale);
        Vector::from([-s[1], -s[2], -s[3], 0.0])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sphere_default_transform() {
        let s = Sphere::new();
        assert_eq!(
            s.transform,
            Motor::from(Transformation::trans_coords(0.0, 0.0, 0.0))
        );
    }

    #[test]
    fn sphere_change_transform() {
        let mut s = Sphere::new();
        let m = Motor::from(Transformation::trans_coords(2.0, 3.0, 4.0));
        s.transform = m;
        assert_eq!(s.transform, m);
    }

    #[test]
    fn sphere_surface_at_point_on_x_axis() {
        let s = Sphere::new();
        assert_eq!(
            s.surface_at(Trivector::point(1.0, 0.0, 0.0)),
            Vector::from([1.0, 0.0, 0.0, 0.0])
        );
    }

    #[test]
    fn sphere_surface_at_point_on_y_axis() {
        let s = Sphere::new();
        assert_eq!(
            s.surface_at(Trivector::point(0.0, 1.0, 0.0)),
            Vector::from([0.0, 1.0, 0.0, 0.0])
        );
    }

    #[test]
    fn sphere_surface_at_point_on_z_axis() {
        let s = Sphere::new();
        assert_eq!(
            s.surface_at(Trivector::point(0.0, 0.0, 1.0)),
            Vector::from([0.0, 0.0, 1.0, 0.0])
        );
    }

    #[test]
    fn sphere_surface_at_nonaxial_point() {
        let s = Sphere::new();
        let r3o3 = f32::sqrt(3.0) / 3.0;
        assert_eq!(
            s.surface_at(Trivector::point(r3o3, r3o3, r3o3)),
            Vector::from([r3o3, r3o3, r3o3, 0.0])
        )
    }

    #[test]
    fn sphere_surface_normalized() {
        let s = Sphere::new();
        let r3o3 = f32::sqrt(3.0) / 3.0;
        let surface = s.surface_at(Trivector::point(r3o3, r3o3, r3o3));
        assert_eq!(surface, surface.normalize())
    }

    #[test]
    fn sphere_surface_translated() {
        let mut s = Sphere::new();
        s.transform(Transformation::trans_coords(0.0, 1.0, 0.0).into());

        let surface = s.surface_at(Trivector::point(0.0, 1.70711, -0.70711));

        assert_eq!(surface, Vector::from([0.0, 0.70711, -0.70711, 0.0]));
    }

    #[test]
    fn sphere_surface_transformed() {
        use std::f32::consts::PI;
        let mut s = Sphere::new();
        s.transform(Transformation::rotation(e12, PI / 5.0).into());
        s.scale = Trivector::scale(1.0, 0.5, 1.0);
        let p = Trivector::point(0.0, f32::sqrt(2.0) / 2.0, -f32::sqrt(2.0) / 2.0);
        assert_eq!(s.surface_at(p), Vector::from([0.0, 0.97014, -0.24254, 0.0]));
    }
}
