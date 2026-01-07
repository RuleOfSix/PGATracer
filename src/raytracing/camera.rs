use crate::raytracing::*;

#[derive(Debug, Clone)]
pub struct Camera {
    pub location: Trivector,
    pub forward: Trivector,
    pub up: Trivector,
    pub hsize: usize,
    pub vsize: usize,
    pub fov: f32,
    pixel_size: f32,
    half_width: f32,
    half_height: f32,
    left: Trivector,
    forward_m: Motor,
}

impl Camera {
    #[inline]
    pub fn new(
        location: Trivector,
        forward: Trivector,
        up: Trivector,
        hsize: usize,
        vsize: usize,
        fov: f32,
    ) -> Self {
        let mut res = Self {
            location,
            forward,
            up,
            hsize,
            vsize,
            fov,
            pixel_size: 0.0,
            half_width: 0.0,
            half_height: 0.0,
            left: Trivector::direction(0.0, 0.0, 0.0),
            forward_m: Motor::from(Transformation::translation(forward)),
        };
        res.update_calculations();
        res
    }

    #[inline]
    pub fn default() -> Self {
        Self {
            location: e123,
            forward: Trivector::direction(0.0, 0.0, -1.0),
            up: Trivector::direction(0.0, 1.0, 0.0),
            hsize: 500,
            vsize: 500,
            fov: std::f32::consts::PI / 2.0,
            pixel_size: 0.004,
            half_width: 1.0,
            half_height: 1.0,
            left: Trivector::direction(-1.0, 0.0, 0.0),
            forward_m: Motor::from(Transformation::translation(-e021)),
        }
    }

    #[inline]
    pub const fn pixel_size(&self) -> f32 {
        self.pixel_size
    }

    #[inline]
    pub fn update_calculations(&mut self) {
        let half_view = f32::tan(self.fov / 2.0);
        let aspect = (self.hsize as f32) / (self.vsize as f32);
        let (half_height, half_width) = if aspect >= 1.0 {
            (half_view / aspect, half_view)
        } else {
            (half_view, half_view * aspect)
        };
        self.pixel_size = (half_width * 2.0) / (self.hsize as f32);
        self.half_height = half_height;
        self.half_width = half_width;
        self.left =
            ((self.up & self.forward).dual().assert::<Bivector>() ^ e0).assert::<Trivector>();
        self.up =
            ((self.forward & self.left).dual().assert::<Bivector>() ^ e0).assert::<Trivector>();
        self.forward_m = Motor::from(Transformation::translation(self.forward));
    }

    #[inline]
    pub fn transform(&mut self, m: Motor) {
        self.location = m >> self.location;
        self.forward = m >> self.forward;
        self.up = m >> self.up;
        self.up[0] = 0.0;
    }

    #[inline]
    pub fn transform_t(&mut self, t: Transformation) {
        let m = Motor::from(t);
        self.location = m >> self.location;
        self.forward = m >> self.forward;
        self.up = m >> self.up;
        self.up[0] = 0.0;
    }

    #[inline]
    pub fn scale(&mut self, s: Trivector) {
        self.location = self.location.scale(s);
    }

    #[inline]
    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let x = x as f32;
        let y = y as f32;
        let x_offset = (x + 0.5) * self.pixel_size - self.half_width;
        let y_offset = self.half_height - (y + 0.5) * self.pixel_size;
        let x_translation = Motor::from(Transformation::translation(x_offset * self.left));
        let y_translation = Motor::from(Transformation::translation(y_offset * self.up));
        let p = x_translation >> (y_translation >> (self.forward_m >> self.location));
        Ray::from((p, p - self.location))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::float_eq;
    use std::f32::consts::PI;

    #[test]
    fn camera_default() {
        use std::f32::consts::PI;
        let c = Camera::default();
        assert_eq!(c.location, e123);
        assert_eq!(c.forward, e021);
        assert_eq!(c.up, -e013);
        assert_eq!(c.hsize, 500);
        assert_eq!(c.vsize, 500);
        assert_eq!(c.fov, PI / 2.0);
    }

    #[test]
    fn pixel_size_horizontal() {
        let camera = Camera::new(e123, e021, -e013, 200, 125, PI / 2.0);
        assert!(float_eq(camera.pixel_size(), 0.01));
    }

    #[test]
    fn pixel_size_vertical() {
        let camera = Camera::new(e123, e021, -e013, 125, 200, PI / 2.0);
        assert!(float_eq(camera.pixel_size(), 0.01));
    }

    #[test]
    fn ray_through_center_canvas() {
        let c = Camera::new(e123, e021, -e013, 201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(c.location, e123);
        assert_eq!(r.forwards(), Trivector::direction(0.0, 0.0, -1.0));
    }

    #[test]
    fn ray_through_corner_canvas() {
        let c = Camera::new(e123, e021, -e013, 201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(c.location, e123);
        assert_eq!(
            r.forwards().normalize(),
            Trivector::direction(0.66519, 0.33259, -0.66851)
        );
    }

    #[test]
    fn ray_through_canvas_when_cam_transformed() {
        use std::f32::consts::SQRT_2;
        let mut c = Camera::new(e123, e021, -e013, 201, 101, PI / 2.0);
        let m1 = Motor::from(Transformation::rotation(e31, PI / 4.0));
        let m2 = Motor::from(Transformation::trans_coords(0.0, -2.0, 5.0));
        c.transform(m1);
        c.transform(m2);
        c.update_calculations();
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(c.location, Trivector::point(0.0, -2.0, 5.0));
        assert_eq!(
            r.forwards(),
            Trivector::direction(SQRT_2 / 2.0, 0.0, -SQRT_2 / 2.0)
        );
    }
}
