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
    pub fn apply_at(&self, point: Trivector) -> Color {
        (self.func)(point)
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
}
