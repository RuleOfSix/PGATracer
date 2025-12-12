pub const fn float_eq(f1: f32, f2: f32) -> bool {
    const EPSILON: f32 = 0.000001;
    return (f1 - f2).abs() < EPSILON;
}
