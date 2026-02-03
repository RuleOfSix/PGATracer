use crate::canvas::*;
use crate::pga_3::*;
use crate::raytracing::geometry::*;
use std::fmt::{Debug, Error, Formatter};
use std::rc::Rc;

#[derive(Clone)]
pub struct Pattern {
    func: Rc<dyn Fn(Trivector) -> Color>,
    transform: Motor,
    scale: Trivector,
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("Pattern")
            .field("func", &"Fn(&Object, Trivector) -> Color")
            .field("transform", &self.transform)
            .field("scale", &self.scale)
            .finish()
    }
}

impl Pattern {
    #[inline]
    pub fn new<F: Fn(Trivector) -> Color + 'static>(func: F) -> Self {
        Self {
            func: Rc::new(func),
            transform: Motor::from(1.0),
            scale: Trivector::scale(1.0, 1.0, 1.0),
        }
    }

    #[inline]
    pub fn blend(p1: &Pattern, p2: &Pattern) -> Self {
        let p1 = p1.clone();
        let p2 = p2.clone();
        Self::new(move |point| {
            (p1.apply_with_transform(point) + p2.apply_with_transform(point)) / 2.0
        })
    }

    #[inline]
    pub fn stripe(c1: Color, c2: Color) -> Self {
        Self::new(move |point| {
            if (point.x().floor() as i32) % 2 == 0 {
                c1
            } else {
                c2
            }
        })
    }

    #[inline]
    pub fn stripe_rec(p1: &Pattern, p2: &Pattern) -> Self {
        let p1 = p1.clone();
        let p2 = p2.clone();
        Self::new(move |point| {
            if (point.x().floor() as i32) % 2 == 0 {
                p1.apply_with_transform(point)
            } else {
                p2.apply_with_transform(point)
            }
        })
    }

    #[inline]
    pub fn ring(c1: Color, c2: Color) -> Self {
        Self::new(move |point| {
            if f32::sqrt(point.x().powi(2) + point.z().powi(2)).floor() as i32 % 2 == 0 {
                c1
            } else {
                c2
            }
        })
    }

    #[inline]
    pub fn ring_rec(p1: &Pattern, p2: &Pattern) -> Self {
        let p1 = p1.clone();
        let p2 = p2.clone();
        Self::new(move |point| {
            if f32::sqrt(point.x().powi(2) + point.z().powi(2)).floor() as i32 % 2 == 0 {
                p1.apply_with_transform(point)
            } else {
                p2.apply_with_transform(point)
            }
        })
    }

    #[inline]
    pub fn checker(c1: Color, c2: Color) -> Self {
        Self::new(move |point| {
            let point = point.snap_to_zero();
            if (point.x().floor() + point.y().floor() + point.z().floor()) as i32 % 2 == 0 {
                c1
            } else {
                c2
            }
        })
    }

    #[inline]
    pub fn checker_rec(p1: &Pattern, p2: &Pattern) -> Self {
        let p1 = p1.clone();
        let p2 = p2.clone();
        Self::new(move |point| {
            let point = point.snap_to_zero();
            if (point.x().floor() + point.y().floor() + point.z().floor()) as i32 % 2 == 0 {
                p1.apply_with_transform(point)
            } else {
                p2.apply_with_transform(point)
            }
        })
    }

    #[inline]
    pub fn gradient_with_smoothing_func<F>(c1: Color, c2: Color, smoothing_func: F) -> Self
    where
        F: Fn(Color, Color, f32) -> Color + 'static,
    {
        Self::new(move |point| smoothing_func(c1, c2, point.x()))
    }

    #[inline]
    pub fn gradient(c1: Color, c2: Color) -> Self {
        Self::gradient_with_smoothing_func(c1, c2, |c1, c2, x| c1 + (c2 - c1) * (x - x.floor()))
    }

    #[inline]
    pub fn apply_at(&self, point: Trivector) -> Color {
        (self.func)(point)
    }

    /// Calculates the color of a pattern at a point, considering the transform of the pattern itself
    /// but not of the shape it's being applied to. This exists primarily to be used by the
    /// recursive patterns' closures.
    #[inline]
    pub fn apply_with_transform(&self, point: Trivector) -> Color {
        let point = (self.transform << point).scale(self.scale.reciprocal());
        self.apply_at(point)
    }

    #[inline]
    pub fn apply_at_shape(&self, shape: ObjectRef, point: Trivector) -> Color {
        let point = (*shape.get_transform() << point).scale(shape.get_scale().reciprocal());
        let point = (self.transform << point).scale(self.scale.reciprocal());
        self.apply_at(point)
    }

    #[inline]
    pub fn transform(&mut self, m: Motor) {
        self.transform = match m * self.transform {
            Versor::Even(m) => m,
            Versor::KVec(AnyKVector::Zero(f)) => Motor::from(f),
            Versor::KVec(AnyKVector::Two(bv)) => Motor::from(bv),
            Versor::KVec(AnyKVector::Four(ps)) => Motor::from(ps),
            _ => panic!("Motor * motor should be motor"),
        };
    }

    #[inline]
    pub fn transform_t(&mut self, t: Transformation) {
        self.transform(t.into());
    }

    #[inline]
    pub fn scale(&mut self, scale: Trivector) {
        self.scale = self.scale.scale(scale);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stripe_constant_y() {
        let pat = Pattern::stripe(WHITE, BLACK);
        assert_eq!(pat.apply_at(Trivector::point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pat.apply_at(Trivector::point(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(pat.apply_at(Trivector::point(0.0, 2.0, 0.0)), WHITE);
    }

    #[test]
    fn stripe_constant_z() {
        let pat = Pattern::stripe(WHITE, BLACK);
        assert_eq!(pat.apply_at(Trivector::point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pat.apply_at(Trivector::point(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pat.apply_at(Trivector::point(0.0, 0.0, 2.0)), WHITE);
    }

    #[test]
    fn stripe_alternates_x() {
        let pat = Pattern::stripe(WHITE, BLACK);
        assert_eq!(pat.apply_at(Trivector::point(-1.1, 0.0, 0.0)), WHITE);
        assert_eq!(pat.apply_at(Trivector::point(-1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pat.apply_at(Trivector::point(-0.1, 0.0, 0.0)), BLACK);
        assert_eq!(pat.apply_at(Trivector::point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pat.apply_at(Trivector::point(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(pat.apply_at(Trivector::point(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pat.apply_at(Trivector::point(2.0, 0.0, 0.0)), WHITE);
    }

    #[test]
    fn stripe_with_pattern_transform() {
        let mut pat = Pattern::stripe(WHITE, BLACK);
        pat.scale(Trivector::scale(2.0, 2.0, 2.0));
        let sphere = Sphere::new();
        assert_eq!(
            pat.apply_at_shape(ObjectRef::Sphere(&sphere), Trivector::point(1.5, 0.0, 0.0)),
            WHITE
        );
    }

    #[test]
    fn stripe_with_object() {
        let pat = Pattern::stripe(WHITE, BLACK);
        let mut sphere = Sphere::new();
        sphere.scale(Trivector::scale(2.0, 2.0, 2.0));
        assert_eq!(
            pat.apply_at_shape(ObjectRef::Sphere(&sphere), Trivector::point(1.5, 0.0, 0.0)),
            WHITE
        );
    }

    #[test]
    fn stripe_with_object_and_pattern_transform() {
        let mut pat = Pattern::stripe(WHITE, BLACK);
        pat.transform_t(Transformation::trans_coords(0.5, 0.0, 0.0));
        let mut sphere = Sphere::new();
        sphere.scale(Trivector::scale(2.0, 2.0, 2.0));
        assert_eq!(
            pat.apply_at_shape(ObjectRef::Sphere(&sphere), Trivector::point(2.5, 0.0, 0.0)),
            WHITE
        );
    }

    #[test]
    fn default_gradient_lerps_x() {
        let pat = Pattern::gradient(WHITE, BLACK);
        assert_eq!(pat.apply_at(Trivector::point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pat.apply_at(Trivector::point(0.25, 0.0, 0.0)), WHITE * 0.75);
        assert_eq!(pat.apply_at(Trivector::point(0.5, 0.0, 0.0)), WHITE * 0.5);
        assert_eq!(pat.apply_at(Trivector::point(0.75, 0.0, 0.0)), WHITE * 0.25);
    }

    #[test]
    fn ring_repeats_x_z() {
        let pat = Pattern::ring(WHITE, BLACK);
        assert_eq!(pat.apply_at(Trivector::point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pat.apply_at(Trivector::point(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pat.apply_at(Trivector::point(1.0, 0.0, 1.0)), BLACK);
        assert_eq!(pat.apply_at(Trivector::point(0.708, 0.0, 0.708)), BLACK);
    }

    #[test]
    fn checkers_repeat_x() {
        let pat = Pattern::checker(WHITE, BLACK);
        assert_eq!(pat.apply_at(Trivector::point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pat.apply_at(Trivector::point(0.99, 0.0, 0.0)), WHITE);
        assert_eq!(pat.apply_at(Trivector::point(1.01, 0.0, 0.0)), BLACK);
    }

    #[test]
    fn checkers_repeat_y() {
        let pat = Pattern::checker(WHITE, BLACK);
        assert_eq!(pat.apply_at(Trivector::point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pat.apply_at(Trivector::point(0.0, 0.99, 0.0)), WHITE);
        assert_eq!(pat.apply_at(Trivector::point(0.0, 1.01, 0.0)), BLACK);
    }

    #[test]
    fn checkers_repeat_z() {
        let pat = Pattern::checker(WHITE, BLACK);
        assert_eq!(pat.apply_at(Trivector::point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pat.apply_at(Trivector::point(0.0, 0.0, 0.99)), WHITE);
        assert_eq!(pat.apply_at(Trivector::point(0.0, 0.0, 1.01)), BLACK);
    }
}
