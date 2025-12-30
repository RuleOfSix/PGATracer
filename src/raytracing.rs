use crate::pga_3::*;
pub use geometry::*;

pub mod geometry;
pub mod lighting;

pub type Ray = Bivector;

#[derive(Debug, Copy, Clone)]
pub struct Intersection<'a> {
    pub t: f32,
    pub obj: ObjectRef<'a>,
}

#[derive(Debug)]
pub struct Camera {
    plane: Vector,
}

impl Camera {
    #[inline]
    pub fn new(v: Vector) -> Self {
        Self { plane: v }
    }

    #[inline]
    pub fn plane(&self) -> Vector {
        self.plane
    }
}

impl Trivector {
    #[inline]
    pub fn reflect(self, surface: Vector) -> Trivector {
        let iv = self.dual().assert::<Vector>();
        (iv - surface * 2.0 * (iv | surface).assert::<Scalar>())
            .undual()
            .assert::<Trivector>()
    }
}

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Intersection) -> bool {
        self.t == other.t && self.obj == other.obj
    }
}

impl<'a> Intersection<'a> {
    fn new(t: f32, obj: ObjectRef<'a>) -> Self {
        Intersection { t, obj }
    }
}

#[macro_export]
macro_rules! intersections {
    ( $(new$xs:tt),* ) => {{
        let mut v = vec![$(Intersection::new$xs, )*];
        v.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).expect("Shouldn't be NaNs or Infs in intersection t values"));
        v
    }};
    ( $($xs:tt),* ) => {{
        let mut v = vec![$($xs, )*];
        v.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).expect("Shouldn't be NaNs or Infs in intersection t values"));
        v
    }}
}

pub trait Hit {
    fn hit(&self) -> Option<&Intersection<'_>>;
}

impl Hit for Vec<Intersection<'_>> {
    fn hit(&self) -> Option<&Intersection<'_>> {
        self.iter().filter(|x| x.t > 0.0).next()
    }
}

impl Ray {
    #[inline]
    pub fn position(&self, t: f32, c: &Camera) -> Trivector {
        (*self ^ c.plane()).assert::<Trivector>() + t * self.forwards()
    }

    #[inline]
    pub fn forwards(&self) -> Trivector {
        Trivector::from([0.0, -self[2], -self[1], -self[0]])
    }

    #[inline]
    pub fn when(&self, p: Trivector, c: &Camera) -> Option<f32> {
        let denom = self
            .forwards()
            .dual()
            .inverse()
            .expect("Inverse of dual of ideal point should exist");
        let origin = (*self ^ c.plane()).assert::<Trivector>().normalize();
        match (origin - p).dual() * denom {
            Versor::KVec(AnyKVector::Zero(f)) => Some(f),
            _ => None,
        }
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
        let c = Camera::new(e1);

        assert_eq!(
            r.position(0.0, &c).normalize(),
            Trivector::point(0.0, 3.0, 4.0)
        );
        assert_eq!(
            r.position(1.0, &c).normalize(),
            Trivector::point(1.0, 3.0, 4.0)
        );
        assert_eq!(
            r.position(-1.0, &c).normalize(),
            Trivector::point(-1.0, 3.0, 4.0)
        );
        assert_eq!(
            r.position(2.5, &c).normalize(),
            Trivector::point(2.5, 3.0, 4.0)
        );
    }

    #[test]
    fn ray_position_inverse_when() {
        let p = Trivector::point(2.0, 3.0, 4.0);
        let d = Trivector::direction(1.0, 0.0, 0.0);
        let r = Ray::from((p, d));
        let c = Camera::new(e1);

        assert_eq!(r.when(r.position(0.0, &c).normalize(), &c), Some(0.0));
        assert_eq!(r.when(r.position(1.0, &c).normalize(), &c), Some(1.0));
        assert_eq!(r.when(r.position(-1.0, &c).normalize(), &c), Some(-1.0));
        assert_eq!(r.when(r.position(2.5, &c).normalize(), &c), Some(2.5));
    }

    #[test]
    fn hit_all_positive() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, ObjectRef::Sphere(&s));
        let i2 = Intersection::new(2.0, ObjectRef::Sphere(&s));
        let xs = vec![i1, i2];
        assert_eq!(xs.hit(), Some(&i1));
    }

    #[test]
    fn hit_some_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, ObjectRef::Sphere(&s));
        let i2 = Intersection::new(1.0, ObjectRef::Sphere(&s));
        let xs = vec![i1, i2];
        assert_eq!(xs.hit(), Some(&i2));
    }

    #[test]
    fn hit_all_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, ObjectRef::Sphere(&s));
        let i2 = Intersection::new(-1.0, ObjectRef::Sphere(&s));
        let xs = vec![i1, i2];
        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn hit_always_lowest_nonnegative() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, ObjectRef::Sphere(&s));
        let i2 = Intersection::new(7.0, ObjectRef::Sphere(&s));
        let i3 = Intersection::new(-3.0, ObjectRef::Sphere(&s));
        let i4 = Intersection::new(2.0, ObjectRef::Sphere(&s));

        let xs = intersections![i1, i2, i3, i4];
        assert_eq!(xs.hit(), Some(&i4));
    }

    #[test]
    fn ray_camera_translation() {
        let p = Trivector::point(1.0, 2.0, 3.0);
        let d = Trivector::direction(0.0, 1.0, 0.0);
        let r = Ray::from((p, d));
        let c = Camera::new(e2 + 2.0 * e0);
        let m = Motor::from(Transformation::trans_coords(3.0, 4.0, 5.0));

        let r2 = m >> r;
        let c2 = Camera::new(m >> c.plane());

        assert_eq!(
            (r2 ^ c2.plane()).normalize(),
            Trivector::point(4.0, 6.0, 8.0).into()
        );
        assert_eq!(r2.forwards(), r.forwards());
    }

    #[test]
    fn ray_camera_scaling() {
        let p = Trivector::point(1.0, 2.0, 3.0);
        let d = Trivector::direction(0.0, 1.0, 0.0);
        let r = Ray::from((p, d));
        let c = Camera::new(e2 + 2.0 * e0);

        let scale = Trivector::scale(2.0, 3.0, 4.0);
        let r2 = r.scale(scale);
        let c2 = Camera::new(c.plane().scale(scale));

        assert_eq!(
            (r2 ^ c2.plane()).normalize(),
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
