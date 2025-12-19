use crate::pga_3::Multivector;

pub type Scalar = f32;

impl Multivector for Scalar {
    fn reverse(&self) -> Self {
        *self
    }
    fn grade_involution(&self) -> Self {
        *self
    }
}
