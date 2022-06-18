//! co2qu.rs — rational ("conic") quadratic Bézier to quadratic Bézier

#[macro_use]
pub(crate) mod pow2;

mod chop;
pub use chop::Chop;
mod consts;
pub use consts::MAX_QUAD_POW2;
mod eval;
mod p3d;
pub use pow2::{BelowQuadTolerance, QuadPowerOf2};

use crate::p3d::Point3Like;
use crate::point;
use crate::types::{Point, QuadSpline};

use glifparser::PointLike;
use log;

use std::cmp;

/// A conic is defined by two end points `start` and `end` and a control point `control`. It is
/// also defined by a weight `weight` (_w_) which is a non-negative number. The weight has a
/// default value of 1. A conic is also known as an ellipse, circle, parabola, or hyperbola
/// depending on the value of the weight and the relative positions of the control, start, and end
/// points.
///
/// <https://pages.mtu.edu/~shene/COURSES/cs3621/NOTES/spline/NURBS/RB-conics.html>
#[derive(Clone, Default)]
pub struct Conic {
    pub start: Point,
    pub end: Point,
    pub control: Point,
    /// The weight of the conic. If _w_==1, parabolic. If _w_ < 1, elliptical. If _w_ > 1,
    /// hyperbolic.
    pub weight: f32,
}

impl Conic {
    pub fn new(start: Point, control: Point, end: Point, weight: f32) -> Self {
        Self {
            start,
            end,
            control,
            weight,
        }
    }
}

/// Defines the type of conic being used.
#[derive(Copy, Clone)]
pub enum ConicKind {
    /// If _w_ is less than 1.
    Ellipse,
    /// If _w_ is exactly equal (within ε) to 1.
    Parabola,
    /// If _w_ is greater than 1.
    Hyperbola,
}

impl Conic {
    pub fn kind(&self) -> ConicKind {
        match self.weight.partial_cmp(&1.0).expect("Failed to compare weight with 1.0") {
            cmp::Ordering::Less => ConicKind::Ellipse,
            cmp::Ordering::Equal => ConicKind::Parabola,
            cmp::Ordering::Greater => ConicKind::Hyperbola,
        }
    }
}

impl Conic {
    fn chop_into_quads_pow2(&self, pow2: &mut usize) -> QuadSpline {
        let quad_count = 1 << *pow2;
        let mut quads: QuadSpline = vec![[Point::default(); 3]; quad_count];
        let conics = self.subdivide(*pow2);
        if *pow2 == MAX_QUAD_POW2 {
            // If an extreme weight generates many quads ...
            let line_start = conics[0].start;
            let line_end = conics[0].end;
            let quad_start = conics[1].start;
            if line_start == line_end && quad_start == conics[1].control {
                quads[0][1] = line_start;
                quads[0][2] = line_start;
                quads[1][0] = line_start; // set ctrl == end to make lines
                quads[1][2] = conics[1].end;
                *pow2 = 1;
                return quads;
            }
        }
        for i in 0..conics.len() {
            let conic = &conics[i];
            quads[i] = [conic.start, conic.control, conic.end];
        }
        for quad in quads.iter_mut() {
            if quad.iter().any(|p| !f32::from(p.x()).is_finite() || !f32::from(p.y()).is_finite()) {
                // if we generated a non-finite, pin ourselves to the middle of the hull,
                // as our first and last are already on the first/last pts of the hull.
                for i in 1..3 {
                    quad[i] = self.control;
                }
            }
        }
        quads
    }
}

impl Conic {
    pub fn as_quads(&self, tol: f32) -> QuadSpline {
        let mut pow2 = self.quad_pow2(tol);
        let orig_pow2 = pow2;
        let ret = self.chop_into_quads_pow2(&mut pow2);
        log::debug!("Tolerance {} yielded QuadSpline of len {}", tol, pow2);
        if orig_pow2 != pow2 {
            log::warn!("Tolerance {} caused lines to be generated, not quads", tol);
        }
        ret
    }
}
