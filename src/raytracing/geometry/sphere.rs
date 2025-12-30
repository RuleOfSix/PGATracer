use super::Sealed;
use crate::intersections;
use crate::pga_3::*;
use crate::raytracing::lighting::*;
use crate::raytracing::*;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub transform: Motor,
    pub scale: Trivector,
    pub material: Material,
}

impl Sealed for Sphere {}
impl Obj for Sphere {
    fn intersect(&self, r: Ray, c: &Camera) -> Vec<Intersection<'_>> {
        let self_t = (self.transform << r).scale(self.scale.reciprocal());
        let c_t = Camera::new((self.transform << c.plane()).scale(self.scale.reciprocal()));

        let m = self_t.normalize() * e123;

        let d_squared = 1.0 - m.grade(3).dual().magnitude().powi(2);

        if d_squared < 0.0 {
            return vec![];
        }

        let p1 = self_t ^ m.grade(1).assert::<Vector>() + e0 * f32::sqrt(d_squared);
        let p2 = self_t ^ (m.grade(1).assert::<Vector>() - e0 * f32::sqrt(d_squared));

        let t1 = self_t.when(p1.assert::<Trivector>().normalize(), &c_t);
        let t2 = self_t.when(p2.assert::<Trivector>().normalize(), &c_t);

        intersections![
            new(t2.expect("t1 should exist"), ObjectRef::Sphere(&self)),
            new(t1.expect("t2 should exist"), ObjectRef::Sphere(&self))
        ]
    }
}

impl Sphere {
    #[inline]
    pub fn new() -> Self {
        Sphere {
            transform: Motor::from([1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
            scale: Trivector::scale(1.0, 1.0, 1.0),
            material: Material::new(),
        }
    }

    #[inline]
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
        let p = (self.transform << p.scale(self.scale.reciprocal())) - e123;
        let mut s = (self.transform >> Vector::from([-p[1], -p[2], -p[3], 0.0]))
            .scale(self.scale)
            .scale_slope(self.scale.reciprocal());
        s[3] = 0.0;
        s.normalize()
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

        dbg!(p);
        dbg!(p.scale(s.scale));
        dbg!((s.transform >> (s.transform << p.scale(s.scale.reciprocal()))).scale(s.scale));
        assert_eq!(s.surface_at(p), Vector::from([0.0, 0.97014, -0.24254, 0.0]));
    }

    #[test]
    fn sphere_default_material() {
        assert_eq!(Sphere::new().material, Material::new());
    }

    #[test]
    fn sphere_material_assignable() {
        let mut s = Sphere::new();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m.clone();
        assert_eq!(s.material, m);
    }

    #[test]
    fn intersect_sphere_twice() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let s = Sphere::new();
        let c = Camera::new(e3 - 5.0 * e0);

        let xs = s.intersect(r, &c);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn intersect_sphere_once() {
        let p = Trivector::point(0.0, 1.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let s = Sphere::new();
        let c = Camera::new(e3 - 5.0 * e0);

        let xs = s.intersect(r, &c);

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

        let xs = s.intersect(r, &c);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_sphere_inside() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let s = Sphere::new();
        let c = Camera::new(e3);

        let xs = s.intersect(r, &c);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn intersect_sphere_behind() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let s = Sphere::new();
        let c = Camera::new(e3 + 5.0 * e0);

        let xs = s.intersect(r, &c);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersect_sets_obj() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let s = Sphere::new();
        let c = Camera::new(e3);

        let xs = s.intersect(r, &c);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].obj, ObjectRef::Sphere(&s));
        assert_eq!(xs[1].obj, ObjectRef::Sphere(&s));
    }

    #[test]
    fn intersect_scaled_sphere() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let c = Camera::new(e3 - 5.0 * e0);

        let mut s = Sphere::new();
        s.scale = Trivector::scale(2.0, 2.0, 2.0);

        let xs = s.intersect(r, &c);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersect_translated_sphere() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let c = Camera::new(e3 - 5.0 * e0);

        let mut s = Sphere::new();
        s.transform = Motor::from(Transformation::trans_coords(5.0, 0.0, 0.0));

        let xs = s.intersect(r, &c);

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

        let xs = s.intersect(r, &c);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }
}
