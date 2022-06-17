// loosely based on Skia's SkFindUnitQuadRoots
fn find_unit_quad_roots(a: f32, b: f32, c: f32, t_values: &mut [f32; 2]) -> usize {
    if a == 0.0 {
        t_values[0] = -c / b;
        return (t_values[0] >= 0f32 && t_values[0] <= 1f32) as usize;
    }
    let dr = (b * b - 4f32 * a * c).sqrt();
    let r = if b < 0.0 { -(b-dr)/2f32 } else { -(b+dr)/2f32 };
    let q = c / r;
    let mut t_count = 0;
    if r != 0.0 {
        let t = r / a;
        if t >= 0.0 && t <= 1.0 {
            t_values[t_count] = t;
            t_count += 1;
        }
    }
    if q != 0.0 {
        let t = q / a;
        if t >= 0.0 && t <= 1.0 {
            t_values[t_count] = t;
            t_count += 1;
        }
    }
    return t_count;
}

impl Extrema for Conic {
    fn extrema(&self) -> f32 {
        let coeff = self.deriv_coeff();
        let mut tValues = [0f32, 0f32];
        let roots = find_unit_quad_roots(coeff[0], coeff[1], coeff[2], &mut tValues);

        tValues[0]
    }
}
