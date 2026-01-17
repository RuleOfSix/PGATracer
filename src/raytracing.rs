use crate::pga_3::*;
use crate::util::float_eq;
pub use camera::*;
pub use geometry::*;

pub mod camera;
pub mod geometry;
pub mod intersections;
pub mod lighting;

pub type Ray = Bivector;

impl Trivector {
    #[inline]
    pub fn reflect(self, surface: Vector) -> Trivector {
        let iv = self.dual().assert::<Vector>();
        (iv - surface * 2.0 * (iv | surface).assert::<Scalar>())
            .undual()
            .assert::<Trivector>()
    }
}

impl Ray {
    #[inline]
    pub fn position(&self, t: f32, c: &Camera) -> Trivector {
        c.location + t * self.forwards()
    }

    #[inline]
    pub fn forwards(&self) -> Trivector {
        Trivector::from([0.0, -self[2], -self[1], -self[0]])
    }

    #[inline]
    pub fn when(&self, p: Trivector, origin: Trivector) -> Option<f32> {
        // This implementation assumes the given point lies on the plane,
        // and will give nonsensical results otherwise
        for (i, f) in self.forwards()[1..4].iter().enumerate() {
            if !float_eq(*f, 0.0) {
                return Some((p - origin)[i + 1] / f);
            }
        }
        return None;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ray_position() {
        let p = Trivector::point(2.0, 3.0, 4.0);
        let d = Trivector::direction(1.0, 0.0, 0.0);
        let r = Ray::from((p, d));
        let c = Camera::new(Trivector::point(2.0, 3.0, 4.0), -e021, -e013, 500, 500, 0.0);

        assert_eq!(
            r.position(0.0, &c).normalize(),
            Trivector::point(2.0, 3.0, 4.0)
        );
        assert_eq!(
            r.position(1.0, &c).normalize(),
            Trivector::point(3.0, 3.0, 4.0)
        );
        assert_eq!(
            r.position(-1.0, &c).normalize(),
            Trivector::point(1.0, 3.0, 4.0)
        );
        assert_eq!(
            r.position(2.5, &c).normalize(),
            Trivector::point(4.5, 3.0, 4.0)
        );
    }

    #[test]
    fn ray_position_inverse_when() {
        let p = Trivector::point(2.0, 3.0, 4.0);
        let d = Trivector::direction(1.0, 0.0, 0.0);
        let r = Ray::from((p, d));
        let c = Camera::new(e123, -e021, -e013, 500, 500, 0.0);

        assert_eq!(
            r.when(r.position(0.0, &c).normalize(), c.location),
            Some(0.0)
        );
        assert_eq!(
            r.when(r.position(1.0, &c).normalize(), c.location),
            Some(1.0)
        );
        assert_eq!(
            r.when(r.position(-1.0, &c).normalize(), c.location),
            Some(-1.0)
        );
        assert_eq!(
            r.when(r.position(2.5, &c).normalize(), c.location),
            Some(2.5)
        );
    }

    #[test]
    fn ray_camera_translation() {
        let p = Trivector::point(1.0, 2.0, 3.0);
        let d = Trivector::direction(0.0, 1.0, 0.0);
        let r = Ray::from((p, d));
        let mut c = Camera::new(p, e013, -e021, 500, 500, 0.0);
        let m = Motor::from(Transformation::trans_coords(3.0, 4.0, 5.0));

        let r2 = m >> r;
        c.transform(m);

        assert_eq!(
            c.location.normalize(),
            Trivector::point(4.0, 6.0, 8.0).into()
        );
        assert_eq!(r2.forwards(), r.forwards());
    }

    #[test]
    fn ray_camera_rotation() {
        use std::f32::consts::PI;
        let p = Trivector::point(1.0, 2.0, 3.0);
        let d = Trivector::direction(0.0, 1.0, 0.0);
        let r = Ray::from((p, d));
        let mut c = Camera::new(p, e013, -e021, 500, 500, 0.0);

        let rotation = Motor::from(Transformation::rotation(e31, PI / 4.0));
        let r2 = rotation >> r;
        c.transform(rotation);

        assert_eq!((c.location.normalize() & r2.normalize()).magnitude(), 0.0);
    }

    #[test]
    fn ray_camera_scaling() {
        let p = Trivector::point(1.0, 2.0, 3.0);
        let d = Trivector::direction(0.0, 1.0, 0.0);
        let r = Ray::from((p, d));
        let mut c = Camera::new(p, e013, -e021, 500, 500, 0.0);

        let scale = Trivector::scale(2.0, 3.0, 4.0);
        let r2 = r.scale(scale);
        c.scale(scale);

        assert_eq!(
            c.location.normalize(),
            Trivector::point(2.0, 6.0, 12.0).into()
        );
        assert_eq!(r2.forwards(), r.forwards() * 3.0);
    }

    #[test]
    fn reflect_incoming_45_deg() {
        let v = Trivector::direction(1.0, -1.0, 0.0);
        let n = Vector::from([0.0, 1.0, 0.0, 0.0]);
        assert_eq!(v.reflect(n), Trivector::direction(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflect_off_slanted() {
        let v = Trivector::direction(0.0, -1.0, 0.0);
        let n = Vector::from([f32::sqrt(2.0) / 2.0, f32::sqrt(2.0) / 2.0, 0.0, 0.0]);
        assert_eq!(v.reflect(n), Trivector::direction(1.0, 0.0, 0.0));
    }
}
