use crate::canvas::*;
use crate::pga_3::*;
use crate::util::float_eq;

#[derive(Debug, PartialEq, Clone)]
pub enum Light {
    Point(PointLight),
}

#[derive(Debug, PartialEq, Clone)]
pub struct PointLight {
    pub position: Trivector,
    pub intensity: Color,
}

impl PointLight {
    #[inline]
    pub const fn new(position: Trivector, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && float_eq(self.ambient, other.ambient)
            && float_eq(self.diffuse, other.diffuse)
            && float_eq(self.specular, other.specular)
            && float_eq(self.shininess, other.shininess)
    }
}

impl Material {
    #[inline]
    pub const fn new() -> Self {
        Material {
            color: WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl Trivector {
    pub fn lighting(
        self,
        m: &Material,
        l: &Light,
        eye: Vector,
        surface: Vector,
        in_shadow: bool,
    ) -> Color {
        #[allow(irrefutable_let_patterns)]
        let Light::Point(l) = l else {
            panic!("Non-point lights not implemented.");
        };
        let color = m.color * l.intensity;
        let mut lightv = l.position - self;
        lightv[0] = 0.0;
        lightv = lightv.normalize();
        let ambient = color * m.ambient;

        if in_shadow {
            return ambient;
        }

        let cos_light_normal = (lightv.dual().assert::<Vector>() | surface).assert::<Scalar>();

        if cos_light_normal < 0.0 {
            return ambient;
        }

        let diffuse = color * m.diffuse * cos_light_normal;
        let reflectv = -lightv.reflect(surface).dual().assert::<Vector>();
        let cos_reflect_eye = (eye | reflectv).assert::<Scalar>();

        if cos_reflect_eye <= 0.0 {
            return ambient + diffuse;
        }

        let specular = l.intensity * cos_reflect_eye.powf(m.shininess) * m.specular;

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn point_light_creation() {
        let white = WHITE;
        let l = PointLight::new(e123, WHITE);
        assert_eq!(l.position, e123);
        assert_eq!(l.intensity, white);
    }

    #[test]
    fn material_defaults() {
        let m = Material::new();
        assert_eq!(m.color, WHITE);
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_eye_between_light_and_surface() {
        let m = Material::new();
        let pos = e123;

        let eye = Trivector::direction(0.0, 0.0, -1.0)
            .dual()
            .assert::<Vector>();
        let surface = Vector::from([0.0, 0.0, -1.0, 0.0]);
        let light = PointLight::new(Trivector::point(0.0, 0.0, -10.0), WHITE);
        assert_eq!(
            pos.lighting(&m, &Light::Point(light), eye, surface, false),
            Color::new(1.9, 1.9, 1.9)
        );
    }

    #[test]
    fn lighting_eye_between_light_and_surface_offset_45() {
        let m = Material::new();
        let pos = e123;

        let eye = Trivector::direction(0.0, f32::sqrt(2.0) / 2.0, -f32::sqrt(2.0) / 2.0);
        let surface = Vector::from([0.0, 0.0, -1.0, 0.0]);
        let light = PointLight::new(Trivector::point(0.0, 0.0, -10.0), WHITE);
        assert_eq!(
            pos.lighting(
                &m,
                &Light::Point(light),
                eye.dual().assert::<Vector>(),
                surface,
                false
            ),
            WHITE
        );
    }

    #[test]
    fn lighting_eye_opposite_surface_light_offset_45() {
        let m = Material::new();
        let pos = e123;

        let eye = Trivector::direction(0.0, 0.0, -1.0)
            .dual()
            .assert::<Vector>();
        let surface = Vector::from([0.0, 0.0, -1.0, 0.0]);
        let light = PointLight::new(Trivector::point(0.0, 10.0, -10.0), WHITE);
        assert_eq!(
            pos.lighting(&m, &Light::Point(light), eye, surface, false),
            Color::new(0.7364, 0.7364, 0.7364)
        );
    }

    #[test]
    fn lighting_eye_in_reflection_path() {
        let m = Material::new();
        let pos = e123;

        let eye = Trivector::direction(0.0, -f32::sqrt(2.0) / 2.0, -f32::sqrt(2.0) / 2.0);
        let surface = Vector::from([0.0, 0.0, -1.0, 0.0]);
        let light = PointLight::new(Trivector::point(0.0, 10.0, -10.0), WHITE);
        assert_eq!(
            pos.lighting(
                &m,
                &Light::Point(light),
                eye.dual().assert::<Vector>(),
                surface,
                false
            ),
            Color::new(1.6364, 1.6364, 1.6364)
        );
    }

    #[test]
    fn lighting_light_behind_surface() {
        let m = Material::new();
        let pos = e123;

        let eye = Trivector::direction(0.0, 0.0, -1.0);
        let surface = Vector::from([0.0, 0.0, -1.0, 0.0]);
        let light = PointLight::new(Trivector::point(0.0, 0.0, 10.0), WHITE);

        assert_eq!(
            pos.lighting(
                &m,
                &Light::Point(light),
                eye.dual().assert::<Vector>(),
                surface,
                false
            ),
            Color::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn lighting_in_shadow() {
        let m = Material::new();
        let pos = e123;

        let eye = Trivector::direction(0.0, 0.0, -1.0);
        let surface = Vector::from([0.0, 0.0, -1.0, 0.0]);
        let light = PointLight::new(Trivector::point(0.0, 0.0, -10.0), WHITE);
        let in_shadow = true;

        let result = pos.lighting(
            &m,
            &Light::Point(light),
            eye.dual().assert::<Vector>(),
            surface,
            in_shadow,
        );
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
