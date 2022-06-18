use super::Conic;
use crate::types::DerivativeCoefficients;
use crate::{point, Point};
use glifparser::PointLike;

impl DerivativeCoefficients<3> for Conic {
    /// Compute the coefficients of the derivative of the conic.
    fn deriv_coeff(&self) -> [f32; 3] {
        let p0 = &self.start;
        let p1 = &self.control;
        let p2 = &self.end;
        let w = self.weight;
        let (a, b, c) = (
            2f32 * p0.x() - 2f32 * p2.x() - 2f32 * p0.x() * w + 2f32 * p2.x() * w,
            -2f32 * p0.x() + 2f32 * p2.x() + 4f32 * p0.x() * w - 4f32 * p1.x() * w,
            -2f32 * p0.x() * w + 2f32 * p1.x() * w,
        );
        return [a.into(), b.into(), c.into()];
    }
}

pub(crate) trait EvalTangentAt {
    fn eval_tangent_at(&self, t: f32) -> Point;
}

impl EvalTangentAt for Conic {
    // translation of SkConic::evalTangentAt
    fn eval_tangent_at(&self, t: f32) -> Point {
        // The derivative equation returns a zero tangent vector when t is 0 or 1,
        // and the control point is equal to the end point.
        // In this case, use the conic endpoints to compute the tangent.
        if (t == 0.0 && self.start == self.control) || (t == 1.0 && self.control == self.end) {
            return point![(self.end.x() - self.start.x()).into(), (self.end.y() - self.start.y()).into()];
        }
        let (s, c, e, k, w, a, _x, _y);
        as_quad_error_setup!(&self, s, c, e, k, w, a, _x, _y);
        let p0 = s.clone();
        let p1 = c.clone();
        let p2 = e.clone();
        let p20 = point![(p2.x() - p0.x()).into(), (p2.y() - p0.y()).into()];
        let p10 = point![(p1.x() - p0.x()).into(), (p1.y() - p0.y()).into()];
        let c = p10 * w;
        let a = w * p20 - p20;
        let b = p20 - (c * 2f32);
        let numer = ((a * t).component_mul(&b)) * t + c;
        let denom = t + t + 1f32;
        return numer / denom;
    }
}
