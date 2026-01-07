use crate::pga_3::*;
use crate::util::float_eq;
use std::simd::Simd;

// Basis: e123, e032, e013, e021
pub type Trivector = KVector<3, 4>;

pub const e123: Trivector = Trivector {
    components: Simd::from_array([1.0, 0.0, 0.0, 0.0]),
};

pub const e032: Trivector = Trivector {
    components: Simd::from_array([0.0, 1.0, 0.0, 0.0]),
};

pub const e013: Trivector = Trivector {
    components: Simd::from_array([0.0, 0.0, 1.0, 0.0]),
};

pub const e021: Trivector = Trivector {
    components: Simd::from_array([0.0, 0.0, 0.0, 1.0]),
};

impl Trivector {
    #[inline]
    pub const fn point(x: f32, y: f32, z: f32) -> Self {
        Self {
            components: Simd::from_array([1.0, -x, -y, -z]),
        }
    }

    #[inline]
    pub const fn direction(x: f32, y: f32, z: f32) -> Self {
        Self {
            components: Simd::from_array([0.0, -x, -y, -z]),
        }
    }

    #[inline]
    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        Self::from([1.0, x, y, z])
    }

    #[inline]
    pub fn reciprocal(self) -> Self {
        let new_components: [f32; 4] = self
            .components
            .as_array()
            .iter()
            .map(|f| if !float_eq(*f, 0.0) { 1.0 / *f } else { *f })
            .collect::<Vec<f32>>()
            .try_into()
            .expect("Should still be four components after mapping");

        Self {
            components: Simd::from(new_components),
        }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        -self[1] / self[0]
    }

    #[inline]
    pub fn y(&self) -> f32 {
        -self[2] / self[0]
    }

    #[inline]
    pub fn z(&self) -> f32 {
        -self[3] / self[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_point() {
        let point = Trivector::point(4.3, -4.2, 3.1);
        assert!(float_eq(point[0], 1.0));
        assert!(float_eq(point[1], -4.3));
        assert!(float_eq(point[2], 4.2));
        assert!(float_eq(point[3], -3.1));
        assert!(!point.is_ideal());
    }

    #[test]
    fn basic_dir() {
        let dir = Trivector::from([0.0, -4.3, 4.2, -3.1]);
        assert!(float_eq(dir[0], 0.0));
        assert!(float_eq(dir[1], -4.3));
        assert!(float_eq(dir[2], 4.2));
        assert!(float_eq(dir[3], -3.1));
        assert!(dir.is_ideal());
    }

    #[test]
    fn point_new() {
        let point = Trivector::point(4.0, -4.0, 3.0);
        assert_eq!(point, Trivector::from([1.0, -4.0, 4.0, -3.0]));
    }

    #[test]
    fn dir_new() {
        let dir = Trivector::direction(4.0, -4.0, 3.0);
        assert_eq!(dir, Trivector::from([0.0, -4.0, 4.0, -3.0]));
    }

    #[test]
    fn trivector_is_grade_3() {
        assert_eq!(Trivector::from([0.0; 4]).highest_grade(), 3);
    }

    #[test]
    fn trivector_first_ideal_component_index() {
        assert_eq!(Trivector::ideal_index(), 1);
    }

    #[test]
    fn trivector_addition() {
        let a1 = Trivector::point(3.0, -2.0, 5.0);
        let a2 = Trivector::direction(-2.0, 3.0, 1.0);
        assert_eq!(a1 + a2, Trivector::from([1.0, -1.0, -1.0, -6.0]));
    }

    #[test]
    fn subtract_points() {
        let p1 = Trivector::point(3.0, 2.0, 1.0);
        let p2 = Trivector::point(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, Trivector::direction(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtract_dir_from_point() {
        let p = Trivector::point(3.0, 2.0, 1.0);
        let p_ideal = Trivector::direction(5.0, 6.0, 7.0);
        assert_eq!(p - p_ideal, Trivector::point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtract_dirs() {
        let d1 = Trivector::direction(3.0, 2.0, 1.0);
        let d2 = Trivector::direction(5.0, 6.0, 7.0);
        assert_eq!(d1 - d2, Trivector::direction(-2.0, -4.0, -6.0));
    }

    #[test]
    fn negation() {
        let a = Trivector::from([1.0, -2.0, 3.0, -4.0]);
        assert_eq!(-a, Trivector::from([-1.0, 2.0, -3.0, 4.0]));
    }

    #[test]
    fn scalar_multiplication() {
        let a = Trivector::from([1.0, -2.0, 3.0, -4.0]);
        assert_eq!(a * 3.5, Trivector::from([3.5, -7.0, 10.5, -14.0]));
    }

    #[test]
    fn scalar_division() {
        let a = Trivector::from([1.0, -2.0, 3.0, -4.0]);
        assert_eq!(a / 2.0, Trivector::from([0.5, -1.0, 1.5, -2.0]));
        assert_eq!(a / 2.0, a * 0.5);
    }

    #[test]
    fn eucl_magnitude() {
        let origin = Trivector::from([1.0, 0.0, 0.0, 0.0]);
        assert_eq!(origin.magnitude(), 1.0);
        assert_eq!(origin.eucl_norm(), 1.0);
        assert_eq!(origin.ideal_norm(), 0.0);
        assert_eq!(Trivector::from([1.0, 2.0, 3.0, 4.0]).magnitude(), 1.0);
        assert_eq!(Trivector::from([-1.0, -2.0, -3.0, -4.0]).magnitude(), 1.0);
    }

    #[test]
    fn ideal_magnitude() {
        let ideal_x = Trivector::from([0.0, -1.0, 0.0, 0.0]);
        let ideal_y = Trivector::from([0.0, 0.0, -1.0, 0.0]);
        let ideal_z = Trivector::from([0.0, 0.0, 0.0, -1.0]);

        assert_eq!(ideal_x.magnitude(), 1.0);
        assert_eq!(ideal_x.ideal_norm(), 1.0);
        assert_eq!(ideal_x.eucl_norm(), 0.0);

        assert_eq!(ideal_y.magnitude(), 1.0);
        assert_eq!(ideal_y.ideal_norm(), 1.0);
        assert_eq!(ideal_y.eucl_norm(), 0.0);
        assert_eq!(ideal_z.magnitude(), 1.0);
        assert_eq!(ideal_z.ideal_norm(), 1.0);
        assert_eq!(ideal_z.eucl_norm(), 0.0);

        assert!(float_eq(
            Trivector::from([0.0, 1.0, 2.0, 3.0]).magnitude(),
            14_f32.sqrt()
        ));
        assert!(float_eq(
            Trivector::from([0.0, -1.0, -2.0, -3.0]).magnitude(),
            14_f32.sqrt()
        ));
    }

    #[test]
    fn normalize() {
        let a = Trivector::direction(4.0, 0.0, 0.0);
        assert_eq!(a.normalize(), Trivector::direction(1.0, 0.0, 0.0));

        let b = Trivector::point(2.0, 3.0, 4.0);
        assert_eq!(b.normalize(), Trivector::point(2.0, 3.0, 4.0));

        let c = Trivector::direction(1.0, 2.0, 3.0);
        let c_mag = f32::sqrt(1.0 + 4.0 + 9.0);
        let c_norm = Trivector::direction(1.0 / c_mag, 2.0 / c_mag, 3.0 / c_mag);
        assert_eq!(c.normalize(), c_norm);
        assert!(float_eq(c.normalize().magnitude(), 1.0));
    }

    #[test]
    fn inner_product_point() {
        assert_eq!(
            Trivector::point(4.5, 5.9, -2.3).inner(Trivector::point(-3.4, 6.0, 1.3)),
            (-1.0).into()
        );

        let a = Trivector::from([3.2, -1.0, -2.0, -3.0]);
        let b = Trivector::from([5.6, 3.0, 2.0, 1.0]);
        assert_eq!(a.inner(b), (-17.92).into());
    }

    #[test]
    fn inner_product_ideal_point() {
        let a = Trivector::direction(-1.0, -2.0, -3.0);
        let b = Trivector::direction(3.0, 2.0, 3.0);
        assert_eq!(a.inner(b), 0.0.into());
    }

    #[test]
    fn inner_product_direction_with_point() {
        let a = Trivector::direction(-1.0, -2.0, -3.0);
        let b = Trivector::point(1.0, 2.0, 3.0);
        assert_eq!(a.inner(b), 0.0.into());
    }

    #[test]
    fn reverse() {
        let tv = Trivector::from([1.0, -2.0, -3.0, -4.0]);
        assert_eq!(tv.reverse(), Trivector::from([-1.0, 2.0, 3.0, 4.0]));
    }
}
