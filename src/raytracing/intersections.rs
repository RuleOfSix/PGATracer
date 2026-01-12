use crate::raytracing::*;

pub trait Hit {
    fn hit(&self) -> Option<&Intersection<'_>>;
}

impl Hit for Vec<Intersection<'_>> {
    fn hit(&self) -> Option<&Intersection<'_>> {
        self.iter().filter(|x| x.t > 0.0).next()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Intersection<'a> {
    t: f32,
    obj: ObjectRef<'a>,
}

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Intersection) -> bool {
        self.t == other.t && self.obj == other.obj
    }
}

impl<'a> Intersection<'a> {
    #[inline]
    pub const fn new(t: f32, obj: ObjectRef<'a>) -> Self {
        Intersection { t, obj }
    }
}

impl Intersection<'_> {
    #[inline]
    pub const fn t(&self) -> f32 {
        self.t
    }

    #[inline]
    pub const fn obj(&self) -> ObjectRef<'_> {
        self.obj
    }

    #[inline]
    pub fn precompute(&self, r: &Ray, c: &Camera) -> IntersectionState<'_> {
        const OVER_ADJUSTMENT: f32 = 10.0 * crate::util::EPSILON;

        let point = r.position(self.t, c).normalize();
        let eyev = r.forwards().undual().assert::<Vector>().normalize();
        let surface = self.obj.surface_at(point);
        let inside = (surface | eyev).assert::<Scalar>() < 0.0;
        let surface = if inside { -surface } else { surface };
        IntersectionState {
            t: self.t,
            obj: self.obj,
            point: point,
            over_point: point - surface.dual().assert::<Trivector>() * OVER_ADJUSTMENT,
            eyev: eyev,
            surface: surface,
            inside: inside,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntersectionState<'a> {
    t: f32,
    obj: ObjectRef<'a>,
    point: Trivector,
    over_point: Trivector,
    eyev: Vector,
    surface: Vector,
    inside: bool,
}

impl IntersectionState<'_> {
    #[inline]
    pub const fn t(&self) -> f32 {
        self.t
    }

    #[inline]
    pub const fn obj(&self) -> ObjectRef<'_> {
        self.obj
    }

    #[inline]
    pub const fn point(&self) -> Trivector {
        self.point
    }

    #[inline]
    pub const fn over_point(&self) -> Trivector {
        self.over_point
    }

    #[inline]
    pub const fn eyev(&self) -> Vector {
        self.eyev
    }

    #[inline]
    pub const fn surface(&self) -> Vector {
        self.surface
    }

    #[inline]
    pub const fn inside(&self) -> bool {
        self.inside
    }
}

#[macro_export]
macro_rules! intersections {
    ( $(new$xs:tt),* ) => {{
        use crate::raytracing::intersections::Intersection;
        let mut v = vec![$(Intersection::new$xs, )*];
        v.sort_unstable_by(|a, b| a.t().partial_cmp(&b.t()).expect("Shouldn't be NaNs or Infs in intersection t values"));
        v
    }};
    ( $($xs:tt),* ) => {{
        let mut v = vec![$($xs, )*];
        v.sort_unstable_by(|a, b| a.t().partial_cmp(&b.t()).expect("Shouldn't be NaNs or Infs in intersection t values"));
        v
    }}
}

#[cfg(test)]
mod test {
    use super::*;

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
    fn precompute_outside() {
        let p = Trivector::point(0.0, 0.0, -5.0);
        let r = Ray::from((
            Trivector::point(0.0, 0.0, -5.0),
            Trivector::direction(0.0, 0.0, 1.0),
        ));
        let s = Sphere::new();
        let c = Camera::new(p, -e021, -e013, 500, 500, 0.0);
        let i = s.intersect(r, &c)[0];
        let comps = i.precompute(&r, &c);

        assert_eq!(comps.t(), i.t());
        assert_eq!(comps.obj(), i.obj());
        assert_eq!(comps.point().normalize(), Trivector::point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev(), Vector::from([0.0, 0.0, -1.0, 0.0]));
        assert_eq!(comps.surface(), Vector::from([0.0, 0.0, -1.0, 0.0]));
        assert_eq!(comps.inside(), false);
    }

    #[test]
    fn precompute_inside() {
        let r = Ray::from((e123, Trivector::direction(0.0, 0.0, 1.0)));
        let s = Sphere::new();
        let c = Camera::new(e123, -e021, -e013, 500, 500, 0.0);
        let i = s.intersect(r, &c)[1];
        let comps = i.precompute(&r, &c);

        assert_eq!(comps.point().normalize(), Trivector::point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev(), Vector::from([0.0, 0.0, -1.0, 0.0]));
        assert_eq!(comps.inside(), true);
        assert_eq!(comps.surface(), Vector::from([0.0, 0.0, -1.0, 0.0]));
    }

    #[test]
    fn precompute_offset_point() {
        use crate::util::EPSILON;

        let p = Trivector::point(0.0, 0.0, -5.0);
        let r = Ray::from((p, Trivector::direction(0.0, 0.0, 1.0)));
        let c = Camera::new(p, -e021, -e013, 500, 500, 0.0);
        let mut shape = Sphere::new();
        shape.transform_t(Transformation::trans_coords(0.0, 0.0, 1.0));
        let shape = Object::Sphere(shape);
        let i = Intersection::new(5.0, (&shape).into());
        let comps = i.precompute(&r, &c);
        assert!(comps.over_point.z() < -EPSILON * 5.0);
        assert!(comps.point.z() > comps.over_point.z());
    }
}
