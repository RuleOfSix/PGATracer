use super::Sealed;
use crate::intersections;
use crate::pga_3::*;
use crate::raytracing::intersections::*;
use crate::raytracing::materials::*;
use crate::raytracing::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Plane {
    pub vector: Vector,
    pub transform: Motor,
    pub scale: Trivector,
    pub material: Material,
}

impl Sealed for Plane {}
impl Obj for Plane {
    #[inline]
    fn local_intersect_from_origin(&self, r: Ray, origin: Trivector) -> Vec<Intersection<'_>> {
        let AnyKVector::Three(intersection) = (r ^ self.vector).normalize() else {
            return vec![];
        };

        if intersection.is_ideal() {
            return vec![];
        }

        let sign = r
            .forwards()
            .dual()
            .assert::<Vector>()
            .inner(intersection.difference(origin).dual().assert::<Vector>())
            .assert::<Scalar>()
            .signum();

        intersections![new(
            (origin & intersection).magnitude() * sign / r.magnitude(),
            ObjectRef::Plane(&self)
        )]
    }

    #[inline]
    fn intersect_from_origin(&self, r: Ray, origin: Trivector) -> Vec<Intersection<'_>> {
        self.local_intersect_from_origin(r, origin)
    }

    #[inline]
    fn local_surface_at(&self, _: Trivector) -> Vector {
        self.vector
    }

    #[inline]
    fn surface_at(&self, _: Trivector) -> Vector {
        self.vector
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
        self.vector = m >> self.vector;
        self.transform = match self.transform * m {
            Versor::KVec(AnyKVector::Zero(f)) => Motor::from(f),
            Versor::KVec(AnyKVector::Two(bv)) => Motor::from(bv),
            Versor::KVec(AnyKVector::Four(ps)) => Motor::from(ps),
            Versor::Even(m) => m,
            _ => panic!("Motor * motor should be motor"),
        }
    }

    #[inline]
    fn transform_t(&mut self, t: Transformation) {
        let m = Motor::from(t);
        self.transform(m);
    }

    #[inline]
    fn get_transform(&self) -> Motor {
        self.transform
    }

    #[inline]
    fn get_scale(&self) -> Trivector {
        self.scale
    }

    #[inline]
    fn set_scale(&mut self, new_scale: Trivector) {
        self.scale = new_scale;
    }

    #[inline]
    fn scale(&mut self, scale: Trivector) {
        self.scale = self.scale.scale(scale);
    }
}

impl From<Vector> for Plane {
    fn from(v: Vector) -> Self {
        Self {
            vector: v,
            transform: Motor::from(1.0),
            scale: Trivector::scale(1.0, 1.0, 1.0),
            material: Material::new(),
        }
    }
}

impl Plane {
    #[inline]
    pub fn new() -> Self {
        Self {
            vector: e2,
            transform: Motor::from(1.0),
            scale: Trivector::scale(1.0, 1.0, 1.0),
            material: Material::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normal_plane_constant() {
        let p = Plane::new();
        let s1 = p.local_surface_at(Trivector::point(0.0, 0.0, 0.0));
        let s2 = p.local_surface_at(Trivector::point(10.0, 0.0, -10.0));
        let s3 = p.local_surface_at(Trivector::point(-5.0, 0.0, 150.0));
        assert_eq!(s1, p.vector);
        assert_eq!(s2, p.vector);
        assert_eq!(s3, p.vector);
    }

    #[test]
    fn intersect_ray_parallel() {
        let p = Plane::new();
        let ray_origin = Trivector::point(0.0, 10.0, 0.0);
        let r = Ray::from((ray_origin, Trivector::direction(0.0, 0.0, 1.0)));
        assert_eq!(p.intersect_from_origin(r, ray_origin), vec![]);
    }

    #[test]
    fn intersect_ray_above() {
        let p = Plane::new();
        let ray_origin = Trivector::point(0.0, 1.0, 0.0);
        let r = Ray::from((ray_origin, Trivector::direction(0.0, -1.0, 0.0)));
        let xs = p.intersect_from_origin(r, ray_origin);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t(), 1.0);
        assert_eq!(xs[0].obj(), ObjectRef::Plane(&p));
    }

    #[test]
    fn intersect_ray_above_diagonal() {
        let p = Plane::new();
        let ray_origin = Trivector::point(-1.0, 1.0, -1.0);
        let r = Ray::from((ray_origin, Trivector::direction(1.0, -1.0, 1.0)));
        let xs = p.intersect_from_origin(r, ray_origin);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t(), 1.0);
        assert_eq!(xs[0].obj(), ObjectRef::Plane(&p));
    }

    #[test]
    fn intersect_ray_above_backwards() {
        let p = Plane::new();
        let ray_origin = Trivector::point(0.0, 1.0, 0.0);
        let r = Ray::from((ray_origin, Trivector::direction(0.0, 1.0, 0.0)));
        let xs = p.intersect_from_origin(r, ray_origin);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t(), -1.0);
        assert_eq!(xs[0].obj(), ObjectRef::Plane(&p));
    }

    #[test]
    fn intersect_ray_below() {
        let p = Plane::new();
        let ray_origin = Trivector::point(0.0, -1.0, 0.0);
        let r = Ray::from((ray_origin, Trivector::direction(0.0, 1.0, 0.0)));
        let xs = p.intersect_from_origin(r, ray_origin);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t(), 1.0);
        assert_eq!(xs[0].obj(), ObjectRef::Plane(&p));
    }

    #[test]
    fn intersect_ray_below_backwards() {
        let p = Plane::new();
        let ray_origin = Trivector::point(0.0, -1.0, 0.0);
        let r = Ray::from((ray_origin, Trivector::direction(0.0, -1.0, 0.0)));
        let xs = p.intersect_from_origin(r, ray_origin);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t(), -1.0);
        assert_eq!(xs[0].obj(), ObjectRef::Plane(&p));
    }
}
