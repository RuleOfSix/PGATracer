use crate::pga_3::*;
pub use geometry::sphere::*;

pub mod geometry;

type Ray = Bivector;

#[derive(Debug)]
pub struct Intersection {
    t: f32,
    obj: Object,
}

impl Intersection {
    fn new(t: f32, obj: Object) -> Self {
        Intersection { t, obj }
    }
}

impl Ray {
    pub fn position(&self, t: f32) -> Trivector {
        ((e123 | *self) * *self).assert::<Trivector>() + (e0 * *self * t).assert::<Trivector>()
    }

    pub fn intersect(self, s: Sphere) -> Vec<Intersection> {
        let m = self * e123;
        let origin = ((e123 | self) * self).assert::<Trivector>();

        let d_squared = 1.0 - m.grade(3).dual().magnitude().powi(2);

        if d_squared < 0.0 {
            return vec![];
        }

        let p = self ^ (m.grade(1).assert::<Vector>() + e0 * f32::sqrt(d_squared));

        let t = (p.assert::<Trivector>() - origin).magnitude();

        return vec![
            Intersection::new(-t, Object::Sphere(s)),
            Intersection::new(t, Object::Sphere(s)),
        ];
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

        assert_eq!(r.position(0.0).normalize(), Trivector::point(0.0, 3.0, 4.0));
        assert_eq!(r.position(1.0).normalize(), Trivector::point(1.0, 3.0, 4.0));
        assert_eq!(
            r.position(-1.0).normalize(),
            Trivector::point(-1.0, 3.0, 4.0)
        );
        assert_eq!(r.position(2.5).normalize(), Trivector::point(2.5, 3.0, 4.0));
    }

    #[test]
    fn intersect_sphere_twice() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let s = Sphere::new();

        let xs = r.intersect(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn intersect_sphere_once() {
        let p = Trivector::point(0.0, 1.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let s = Sphere::new();

        let xs = r.intersect(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 0.0);
        assert_eq!(xs[1].t, 0.0);
    }

    #[test]
    fn intersect_sphere_none() {
        let p = Trivector::point(0.0, 2.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let s = Sphere::new();

        let xs = r.intersect(s);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_sets_obj() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let s = Sphere::new();

        let xs = r.intersect(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].obj, Object::Sphere(s));
        assert_eq!(xs[1].obj, Object::Sphere(s));
    }
}
