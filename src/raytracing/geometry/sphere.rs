use super::Sealed;
use crate::intersections;
use crate::pga_3::*;
use crate::raytracing::intersections::*;
use crate::raytracing::lighting::*;
use crate::raytracing::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    pub transform: Motor,
    pub scale: Trivector,
    pub material: Material,
}

impl Sealed for Sphere {}
impl Obj for Sphere {
    fn intersect(&self, r: Ray, c: &Camera) -> Vec<Intersection<'_>> {
        // let r_t = (self.transform << r).scale(self.scale.reciprocal());
        let origin = (self.transform << c.location).scale(self.scale.reciprocal());
        let r_t = ((origin + (self.transform << r.forwards()).scale(self.scale.reciprocal()))
            & origin)
            .assert::<Bivector>();

        let ov = r_t.normalize() * e123;

        let d_squared = 1.0 - ov.grade(3).dual().magnitude().powi(2);

        if d_squared < 0.0 {
            return vec![];
        }

        let p1 = r_t ^ (ov.grade(1).assert::<Vector>() + e0 * f32::sqrt(d_squared));
        let p2 = r_t ^ (ov.grade(1).assert::<Vector>() - e0 * f32::sqrt(d_squared));

        let t1 = r_t.when(p1.assert::<Trivector>().normalize(), origin);
        let t2 = r_t.when(p2.assert::<Trivector>().normalize(), origin);

        intersections![
            new(t2.expect("t2 should exist"), ObjectRef::Sphere(&self)),
            new(t1.expect("t1 should exist"), ObjectRef::Sphere(&self))
        ]
    }

    #[inline]
    fn surface_at(&self, p: Trivector) -> Vector {
        let p = (self.transform << p).scale(self.scale.reciprocal()) - e123;
        let mut s = self.transform
            >> Vector::from([-p[1], -p[2], -p[3], 0.0])
                .scale(self.scale)
                .scale_slope(self.scale.reciprocal());
        s[3] = 0.0;
        s.normalize()
    }

    #[inline]
    fn material(&self) -> &Material {
        &self.material
    }

    #[inline]
    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    #[inline]
    fn set_material(&mut self, m: Material) {
        self.material = m;
    }

    #[inline]
    fn transform(&mut self, m: Motor) {
        self.transform = match self.transform * m {
            Versor::Even(m) => m,
            Versor::KVec(AnyKVector::Zero(s)) => Motor::from(s),
            Versor::KVec(AnyKVector::Two(bv)) => Motor::from(bv),
            Versor::KVec(AnyKVector::Four(ps)) => Motor::from(ps),
            _ => panic!("motor * motor should = motor"),
        };
    }

    #[inline]
    fn transform_t(&mut self, t: Transformation) {
        let m = Motor::from(t);
        self.transform(m);
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
    pub fn normalize(&mut self) {
        self.transform = self.transform.normalize();
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

        assert_eq!(
            s.surface_at(p),
            Vector::from([0.41499, 0.86207, -0.29089, 0.0])
        );
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
        let c = Camera::new(p, -e013, -e021, 500, 500, 0.0);

        let xs = s.intersect(r, &c);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), 4.0);
        assert_eq!(xs[1].t(), 6.0);
    }

    #[test]
    fn intersect_sphere_once() {
        let p = Trivector::point(0.0, 1.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let s = Sphere::new();
        let c = Camera::new(p, -e013, -e021, 500, 500, 0.0);

        let xs = s.intersect(r, &c);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), 5.0);
        assert_eq!(xs[1].t(), 5.0);
    }

    #[test]
    fn intersect_sphere_none() {
        let p = Trivector::point(0.0, 1.2, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let mut s = Sphere::new();
        s.transform(Transformation::rotation(e23, 3.141592 / 4.0).into());
        let c = Camera::new(p, -e013, -e021, 500, 500, 0.0);

        let xs = s.intersect(r, &c);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_sphere_inside() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let s = Sphere::new();
        let c = Camera::default();

        let xs = s.intersect(r, &c);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), -1.0);
        assert_eq!(xs[1].t(), 1.0);
    }

    #[test]
    fn intersect_sphere_behind() {
        let p = Trivector::point(0.0, 0.0, 5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let s = Sphere::new();
        let c = Camera::new(p, -e021, -e013, 500, 500, 0.0);

        let xs = s.intersect(r, &c);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), -6.0);
        assert_eq!(xs[1].t(), -4.0);
    }

    #[test]
    fn intersect_sets_obj() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let s = Sphere::new();
        let c = Camera::default();

        let xs = s.intersect(r, &c);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].obj(), ObjectRef::Sphere(&s));
        assert_eq!(xs[1].obj(), ObjectRef::Sphere(&s));
    }

    #[test]
    fn intersect_scaled_sphere() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let c = Camera::new(p, -e021, -e013, 500, 500, 0.0);

        let mut s = Sphere::new();
        s.scale = Trivector::scale(2.0, 2.0, 2.0);

        let xs = s.intersect(r, &c);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), 3.0);
        assert_eq!(xs[1].t(), 7.0);
    }

    #[test]
    fn intersect_translated_sphere() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let c = Camera::new(p, -e021, -e013, 500, 500, 0.0);

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
        let c = Camera::new(p, -e021, -e013, 500, 500, 0.0);

        let mut s = Sphere::new();
        s.transform_t(Transformation::rotation(e23, 3.141592 / 4.0));

        let xs = s.intersect(r, &c);

        assert_eq!(xs.len(), 2);
        assert!(f32::abs(xs[0].t() - 4.0) < 0.01);
        assert!(f32::abs(xs[1].t() - 6.0) < 0.01);
    }

    #[test]
    fn intersect_flattened_sphere() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let c = Camera::new(p, -e021, -e013, 100, 100, std::f32::consts::PI / 2.0);

        let mut s = Sphere::new();
        s.scale = Trivector::scale(10.0, 10.0, 0.1);

        let xs = s.intersect(r, &c);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), 4.9);
        assert_eq!(xs[1].t(), 5.1);
    }

    #[test]
    fn intersect_flattened_translated_sphere() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let d = Trivector::direction(0.0, 0.0, 1.0);
        let r = Ray::from((p, d));
        let c = Camera::new(p, -e021, -e013, 100, 100, std::f32::consts::PI / 2.0);

        let mut s = Sphere::new();
        s.scale = Trivector::scale(10.0, 10.0, 0.1);
        s.transform_t(Transformation::trans_coords(3.0, 3.0, 0.0));

        let xs = s.intersect(r, &c);

        assert_eq!(xs.len(), 2);
        assert!(f32::abs(xs[0].t() - 4.9) < 0.01);
        assert!(f32::abs(xs[1].t() - 5.1) < 0.01);
    }

    #[test]
    fn intersect_scaled_sphere_edge_case_1() {
        let r = Ray::from([
            0.9950371,
            -0.16532159,
            0.3251585,
            -0.6659479,
            1.6257925,
            0.48773777,
        ]);

        use std::f32::consts::PI;
        let cam_loc = Trivector::point(0.0, 1.5, -5.0);
        let cam_target = Trivector::point(0.0, 1.0, 0.0);

        let c = Camera::new(
            cam_loc,
            (cam_target - cam_loc).normalize(),
            -e013,
            500,
            500,
            PI / 3.0,
        );

        let mut right = Sphere::new();
        right.transform_t(Transformation::trans_coords(1.5, 0.5, -0.5));
        right.scale = Trivector::scale(0.5, 0.5, 0.5);

        let xs = right.intersect(r, &c);

        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn intersect_scaled_sphere_edge_case_2() {
        let r = Ray::from([
            0.9950371,
            -0.2969575,
            -0.047107764,
            -0.007768154,
            -0.23553883,
            -0.07066165,
        ]);

        use std::f32::consts::PI;
        let cam_loc = Trivector::point(0.0, 1.5, -5.0);
        let cam_target = Trivector::point(0.0, 1.0, 0.0);

        let c = Camera::new(
            cam_loc,
            (cam_target - cam_loc).normalize(),
            -e013,
            500,
            500,
            PI / 3.0,
        );

        let mut floor = Sphere::new();
        floor.scale = Trivector::scale(10.0, 0.01, 1.0);

        let xs = floor.intersect(r, &c);

        assert_eq!(xs.len(), 2);
    }
}
