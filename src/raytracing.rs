use crate::pga_3::*;
pub use geometry::*;

pub mod geometry;

pub type Ray = Bivector;

#[derive(Debug, Copy, Clone)]
pub struct Intersection<'a> {
    t: f32,
    obj: Object<'a>,
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

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Intersection) -> bool {
        self.t == other.t && self.obj == other.obj
    }
}

impl<'a> Intersection<'a> {
    fn new(t: f32, obj: Object<'a>) -> Self {
        Intersection { t, obj }
    }
}

pub trait Hit<'a> {
    fn hit(&self) -> Option<Intersection<'a>>;
}

impl<'a> Hit<'a> for Vec<Intersection<'a>> {
    fn hit(&self) -> Option<Intersection<'a>> {
        self.iter().fold(None, |acc: Option<Intersection<'a>>, i| {
            if i.t > 0.0 && if let Some(a) = acc { i.t < a.t } else { true } {
                Some(*i)
            } else {
                acc
            }
        })
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

    pub fn intersect<'a>(self, s: &'a Sphere, c: &Camera) -> Vec<Intersection<'a>> {
        let self_t = s.transform << self.scale(s.scale.reciprocal());
        let c_t = Camera::new(s.transform << c.plane().scale(s.scale.reciprocal()));

        let m = self_t.normalize() * e123;

        let d_squared = 1.0 - m.grade(3).dual().magnitude().powi(2);

        if d_squared < 0.0 {
            return vec![];
        }

        let p1 = self_t ^ m.grade(1).assert::<Vector>() + e0 * f32::sqrt(d_squared);
        let p2 = self_t ^ (m.grade(1).assert::<Vector>() - e0 * f32::sqrt(d_squared));

        let t1 = self_t.when(p1.assert::<Trivector>().normalize(), &c_t);
        let t2 = self_t.when(p2.assert::<Trivector>().normalize(), &c_t);

        vec![
            Intersection::new(t1.expect("t1 should exist"), Object::Sphere(s)),
            Intersection::new(t2.expect("t2 should exist"), Object::Sphere(s)),
        ]
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
    fn intersect_sphere_twice() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let s = Sphere::new();
        let c = Camera::new(e3 - 5.0 * e0);

        let xs = r.intersect(&s, &c);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 6.0);
        assert_eq!(xs[1].t, 4.0);
    }

    #[test]
    fn intersect_sphere_once() {
        let p = Trivector::point(0.0, 1.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let s = Sphere::new();
        let c = Camera::new(e3 - 5.0 * e0);

        let xs = r.intersect(&s, &c);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn intersect_sphere_none() {
        let p = Trivector::point(0.0, 1.2, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let mut s = Sphere::new();
        s.transform(Transformation::rotation(e23, 3.141592 / 4.0).into());
        let c = Camera::new(e3 - 5.0 * e3);

        let xs = r.intersect(&s, &c);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_sphere_inside() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let s = Sphere::new();
        let c = Camera::new(e3);

        let xs = r.intersect(&s, &c);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, -1.0);
    }

    #[test]
    fn intersect_sphere_behind() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let s = Sphere::new();
        let c = Camera::new(e3 + 5.0 * e0);

        let xs = r.intersect(&s, &c);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -4.0);
        assert_eq!(xs[1].t, -6.0);
    }

    #[test]
    fn intersect_sets_obj() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let s = Sphere::new();
        let c = Camera::new(e3);

        let xs = r.intersect(&s, &c);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].obj, Object::Sphere(&s));
        assert_eq!(xs[1].obj, Object::Sphere(&s));
    }

    #[test]
    fn hit_all_positive() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, Object::Sphere(&s));
        let i2 = Intersection::new(2.0, Object::Sphere(&s));
        let xs = vec![i1, i2];
        assert_eq!(xs.hit(), Some(i1));
    }

    #[test]
    fn hit_some_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, Object::Sphere(&s));
        let i2 = Intersection::new(1.0, Object::Sphere(&s));
        let xs = vec![i1, i2];
        assert_eq!(xs.hit(), Some(i2));
    }

    #[test]
    fn hit_all_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, Object::Sphere(&s));
        let i2 = Intersection::new(-1.0, Object::Sphere(&s));
        let xs = vec![i1, i2];
        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn hit_always_lowest_nonnegative() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, Object::Sphere(&s));
        let i2 = Intersection::new(7.0, Object::Sphere(&s));
        let i3 = Intersection::new(-3.0, Object::Sphere(&s));
        let i4 = Intersection::new(2.0, Object::Sphere(&s));

        let xs = vec![i1, i2, i3, i4];
        assert_eq!(xs.hit(), Some(i4));
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
    fn intersect_scaled_sphere() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let c = Camera::new(e3 - 5.0 * e0);

        let mut s = Sphere::new();
        s.scale = Trivector::scale(2.0, 2.0, 2.0);

        let xs = r.intersect(&s, &c);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 7.0);
        assert_eq!(xs[1].t, 3.0);
    }

    #[test]
    fn intersect_translated_sphere() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let c = Camera::new(e3 - 5.0 * e0);

        let mut s = Sphere::new();
        s.transform = Motor::from(Transformation::trans_coords(5.0, 0.0, 0.0));

        let xs = r.intersect(&s, &c);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_rotated_sphere() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let c = Camera::new(e3 - 5.0 * e0);

        let mut s = Sphere::new();
        s.transform(Transformation::rotation(e12, 3.141592 / 2.0).into());

        let xs = r.intersect(&s, &c);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 6.0);
        assert_eq!(xs[1].t, 4.0);
    }
}
