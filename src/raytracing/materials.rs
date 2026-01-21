use crate::canvas::{Color, WHITE};
use crate::util::float_eq;

pub mod patterns;
use patterns::Pattern;

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub pattern: Option<Pattern>,
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
    pub fn new() -> Self {
        Material {
            color: WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
        }
    }
}
