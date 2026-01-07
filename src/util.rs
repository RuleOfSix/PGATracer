pub const fn float_eq(f1: f32, f2: f32) -> bool {
    const EPSILON: f32 = 0.001;
    return (f1 - f2).abs() < EPSILON;
}

pub fn sum_of_squares(floats: &[f32]) -> f32 {
    floats
        .iter()
        .map(|f| f.powi(2))
        .fold(0.0, |acc, f| acc + f)
        .sqrt()
}
