use super::*;
use crate::types::DerivativeCoefficients;

use std::f32::EPSILON;
pub const EPSILON_F32: f32 = EPSILON * 10.;

use num_traits::Float;

/// The trait allowing the curve to be subdivided into smaller pieces.
pub trait Chop
where
    Self: Clone + Default + Sized,
{
    /// divide curve into two at t
    fn chop_at(&self, t: f32, dst: &mut [Self; 2]) -> bool;
    /// get a subsection of the curve from t1…t2
    fn chop_at_t2(&self, t1: f32, t2: f32) -> Self;
    /// splits the curve into `2^level+1` linear pieces.
    fn subdivide(&self, level: usize) -> Vec<Self>;
    /// chop into 2 conics, with the first conic be the portion from t=0.0 to t=0.5, and the second going from t=0.5 to t=1.0
    fn chop(&self) -> [Self; 2] {
        [self.chop_at_t2(0.0, 0.5), self.chop_at_t2(0.5, 1.0)]
    }
}

impl Chop for Conic {
    fn chop_at(&self, t: f32, dst: &mut [Conic; 2]) -> bool {
        let src = [self.start.clone(), self.control.clone(), self.end.clone()];
        let tmp = p3d::ratquad_map(&src, self.weight);
        let tmp2 = p3d::interp(&tmp, t.into());
        dst[0].start = self.start.clone();
        dst[0].control = tmp2[0].into();
        dst[0].end = tmp2[1].into();
        dst[1].start = dst[0].end.clone();
        dst[1].control = tmp2[2].into();
        dst[1].end = self.end.clone();
        let root = Float::sqrt(tmp2[1].z());
        dst[0].weight = (tmp2[0].z() / root).into();
        dst[1].weight = (tmp2[1].z() / root).into();
        return dst.into_iter().all(|c| {
            [&c.start, &c.control, &c.end]
                .into_iter()
                .all(|p3| f32::from(p3.x()).is_finite() && f32::from(p3.y()).is_finite())
        });
    }

    fn chop_at_t2(&self, t1: f32, t2: f32) -> Self {
        let mut dst: Conic = Default::default();
        if t1 < EPSILON_F32 || t2 > (1.0f32 - EPSILON_F32) {
            if t1 < EPSILON_F32 && t2 > (1.0f32 - EPSILON_F32) {
                dst = self.clone();
                return dst;
            } else {
                let mut pair = [Conic::default(), Conic::default()];
                if self.chop_at(if t1 > EPSILON_F32 { t1 } else { t2 }, &mut pair) {
                    return pair[if t1 >= EPSILON_F32 { 0 } else { 1 }].clone();
                }
            }
        }
        let coeff = self.deriv_coeff();
        let coeff = (coeff[0], coeff[1], coeff[2]);
        let tt1 = (t1 as f32, t1 as f32);
        let a_xy = (coeff.0 * tt1.0 + coeff.1) * tt1.0 + coeff.2;
        let a_zz = (coeff.0 + coeff.0) * tt1.0 + 1f32;
        let mid_tt = ((t1 * 2f32 + t2) * 0.5f32, (t1 * 2f32 + t2) * 0.5f32);
        let d_xy = (coeff.0 * mid_tt.0 + coeff.1) * mid_tt.0 + coeff.2;
        let d_zz = (coeff.0 + coeff.0) * mid_tt.0 + 1f32;
        let tt2 = (t2 as f32, t2 as f32);
        let c_xy = (coeff.0 * tt2.0 + coeff.1) * tt2.0 + coeff.2;
        let c_zz = (coeff.0 + coeff.0) * tt2.0 + 1f32;
        let b_xy = d_xy * 2f32 - (a_xy + c_xy) * 0.5f32;
        let b_zz = d_zz * 2f32 - (a_zz + c_zz) * 0.5f32;
        dst.start = point![a_xy / a_zz, a_xy / a_zz];
        dst.control = point![b_xy / b_zz, b_xy / b_zz];
        dst.end = point![c_xy / c_zz, c_xy / c_zz];
        let ww = b_zz / (a_zz * c_zz).sqrt();
        dst.weight = ww;
        dst
    }

    fn subdivide(&self, level: usize) -> Vec<Self> {
        if level == 0 {
            return vec![self.clone()];
        }
        let dst = self.chop();
        let mut ret = Vec::new();
        ret.extend(dst[0].subdivide(level - 1));
        ret.extend(dst[1].subdivide(level - 1));
        ret
    }
}
