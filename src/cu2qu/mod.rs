//! cu2qu.rs — cubic Bézier to quadratic Bézier
use std::{error::Error, fmt};

use crate::point;
use crate::types::{Cubic, Point, Quad, QuadSpline};

// We won't divide any cubic over 100 times
const MAX_N: usize = 100;

/// `Coefficients` trait is implemented by types that can be converted to their respective coefficients _and back to points_.
trait Coefficients {
    fn points(&self) -> Self;
    fn coefficients(&self) -> Self;
}

impl Coefficients for Cubic {
    fn points(&self) -> [Point; 4] {
        let [a, b, c, d] = self;
        let _1 = *d;
        let _2 = (*c / 3.0) + *d;
        let _3 = (*b + *c) / 3.0 + _2;
        let _4 = *a + *d + *c + *b;
        [_1, _2, _3, _4]
    }

    fn coefficients(&self) -> [Point; 4] {
        let [p0, p1, p2, p3] = self;
        let c = (*p1 - *p0) * 3.0;
        let b = (*p2 - *p1) * 3.0 - c;
        let d = *p0;
        let a = *p3 - d - c - b;
        [a, b, c, d]
    }
}

/// “Could not approximate cubic curve with a quadratic”
#[derive(Debug)]
pub struct ApproxNotFoundError;

impl fmt::Display for ApproxNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApproxNotFoundError: Could not approximate cubic curve with a quadratic")?;
        Ok(())
    }
}

impl Error for ApproxNotFoundError {
    fn description(&self) -> &str {
        "Could not approximate cubic curve with a quadratic"
    }
}

trait SplitCubic {
    /// Splits cubic curve into n equal parts
    fn split_into_n(&self, n: usize) -> Vec<Cubic>;
}

/// Cubic Bézier approximation with N quadratic splines
pub trait CubicApprox {
    /// Calculates the approximate control point of a cubic Bézier curve to a quadratic Bézier.
    fn approx_control(&self, t: f32) -> Point;
    /// Returns a quadratic representation with a tolerance error.
    fn approx_quadratic(&self, tolerance: f32) -> Result<Quad, ApproxNotFoundError>;
    /// Returns an array of quadratic spline segments with a tolerance error.
    fn approx_spline(&self, n: usize, tolerance: f32) -> Result<QuadSpline, ApproxNotFoundError>;
}

/// Trait that returns true if the control point of a quadratic Bézier curve stays in the bounding box of the cubic Bézier curve.
pub trait CubicFarthestFitInside {
    fn farthest_fit_inside(&self, tolerance: f32) -> bool;
}

impl SplitCubic for Cubic {
    fn split_into_n(&self, n: usize) -> Vec<Cubic> {
        let [a, b, c, d] = (*self).coefficients();
        let dt = 1f32 / n as f32;
        let delta_2 = dt * dt;
        let delta_3 = dt * delta_2;
        (0..n)
            .map(|i| {
                let t1 = i as f32 * dt;
                let t1_2 = t1 * t1;
                let a1 = a * delta_3;
                let b1 = (3f32 * a * t1 + b) * delta_2;
                let c1 = (2f32 * b * t1 + c + 3f32 * a * t1_2) * dt;
                let d1 = a * t1 * t1_2 + b * t1_2 + c * t1 + d;
                [a1, b1, c1, d1].points()
            })
            .collect()
    }
}

impl CubicApprox for Cubic {
    fn approx_control(&self, t: f32) -> Point {
        let [p0, p1, p2, p3] = self;
        let _p1 = *p0 + (*p1 - *p0) * 1.5;
        let _p2 = *p3 + (*p2 - *p3) * 1.5;
        _p1 + (_p2 - _p1) * t
    }
    fn approx_quadratic(&self, tolerance: f32) -> Result<Quad, ApproxNotFoundError> {
        let q1 = calc_intersect(self[0], self[1], self[2], self[3]);
        if q1[1].is_nan() {
            return Err(ApproxNotFoundError);
        }
        let c0 = self[0];
        let c3 = self[3];
        let c1 = c0 + (q1 - c0) * (2.0 / 3.0);
        let c2 = c3 + (q1 - c3) * (2.0 / 3.0);
        let cubic = [point![0.0, 0.0], c1 - self[1], c2 - self[2], point![0.0, 0.0]];
        if !cubic.farthest_fit_inside(tolerance) {
            return Err(ApproxNotFoundError);
        }
        Ok([c0, q1, c3])
    }
    fn approx_spline(&self, n: usize, tolerance: f32) -> Result<QuadSpline, ApproxNotFoundError> {
        if n == 1 {
            return self.approx_quadratic(tolerance).map(|ok| vec![ok]);
        }
        let mut cubics = self.split_into_n(n).into_iter();
        let mut next_cubic = cubics.next().unwrap();
        let mut next_q1 = next_cubic.approx_control(0f32);
        let mut q2 = self[0];
        let mut d1 = point![0f32, 0f32];
        let mut spline: Vec<Point> = vec![self[0]];
        for i in 1..(n + 1) {
            let _c0 = next_cubic[0];
            let c1 = next_cubic[1];
            let c2 = next_cubic[2];
            let c3 = next_cubic[3];
            let q0 = q2;
            let q1 = next_q1;
            (next_cubic, next_q1, q2) = if i < n {
                next_cubic = cubics.next().unwrap();
                next_q1 = next_cubic.approx_control(i as f32 / (n - 1) as f32);
                q2 = (q1 + next_q1) * 0.5;
                spline.push(next_q1);
                (next_cubic, next_q1, q2)
            } else {
                (next_cubic, next_q1, c3)
            };
            let d0 = d1;
            d1 = q2 - c3;
            if d1.norm_squared() > tolerance
                || ![d0, q0 + (q1 - q0) * (2.0 / 3.0) - c1, q2 + (q1 - q2) * (2.0 / 3.0) - c2, d1].farthest_fit_inside(tolerance)
            {
                return Err(ApproxNotFoundError);
            }
        }
        spline.push(self[3]);
        let mut splines = vec![];
        let mut iter = spline.iter().peekable();
        let mut prev = iter.next().unwrap();
        let mut cur_spline = vec![];
        while let Some(n) = iter.next() {
            let p_ = iter.peek().map(|p| p.clone());
            if let Some(p) = p_ {
                cur_spline.push(prev);
                cur_spline.push(n);
                cur_spline.push(p);
            } else {
                break;
            };
            splines.push(cur_spline);
            cur_spline = vec![];
            prev = splines.last().unwrap().last().unwrap();
        }
        Ok(splines.into_iter().map(|q| [*q[0], *q[1], *q[2]]).collect())
    }
}

/// Calculate point of intersection between two curves.
///
/// Returns a `Point` representing the intersection of two quadratic Bézier curves.
fn calc_intersect(a: Point, b: Point, c: Point, d: Point) -> Point {
    let ab = b - a;
    let cd = d - c;
    let p = ab.component_mul(&point![0f32, 1f32]);
    let h = p.dot(&(a - c)) / p.dot(&cd);
    c + cd * h
}

impl CubicFarthestFitInside for Cubic {
    fn farthest_fit_inside(&self, tolerance: f32) -> bool {
        let p0 = self[0];
        let p1 = self[1];
        let p2 = self[2];
        let p3 = self[3];
        if p2.norm_squared() <= tolerance && p1.norm_squared() <= tolerance {
            return true;
        }

        let mid = (p0 + 3f32 * (p1 + p2) + p3) * 0.125;
        if mid.norm_squared() > tolerance {
            return false;
        }
        let deriv3 = (p3 + p2 - p1 - p0) * 0.125;
        let inside1 = Cubic::farthest_fit_inside(&[p0, (p0 + p1) * 0.5, mid - deriv3, mid], tolerance);
        let inside2 = Cubic::farthest_fit_inside(&[mid, mid + deriv3, (p2 + p3) * 0.5, p3], tolerance);
        inside1 && inside2
    }
}

/// Convert a cubic Bézier curve to a quadratic spline segment.
pub trait CurveToQuadratic {
    fn curve_to_quadratic(&self, max_err: f32) -> Result<QuadSpline, ApproxNotFoundError>;
}

impl CurveToQuadratic for Cubic {
    fn curve_to_quadratic(&self, max_err: f32) -> Result<QuadSpline, ApproxNotFoundError> {
        for n in 1..(MAX_N + 1) {
            let spline = self.approx_spline(n, max_err);
            if let Ok(spl) = spline {
                return Ok(spl);
            }
        }
        Err(ApproxNotFoundError)
    }
}

/// Convert a vector of cubic Bézier curves to a vector of quadratic spline segments.
pub trait CurvesToQuadratic {
    fn curves_to_quadratic(&self, max_errors: Vec<f32>) -> Result<Vec<QuadSpline>, ApproxNotFoundError>;
}

impl CurvesToQuadratic for Vec<Cubic> {
    fn curves_to_quadratic(&self, max_errors: Vec<f32>) -> Result<Vec<QuadSpline>, ApproxNotFoundError> {
        debug_assert_eq!(self.len(), max_errors.len());
        let l = self.len();
        let mut splines = vec![None; l];
        let mut last_i = 0;
        let mut i = 0;
        let mut n = 1;
        loop {
            let test_spline = self[i].approx_spline(n, max_errors[i]);
            if let Err(_) = test_spline {
                if n == MAX_N {
                    break;
                };
                n += 1;
                last_i = i;
                continue;
            }
            splines[i] = match test_spline {
                Ok(a) => Some(a),
                Err(e) => return Err(e),
            };
            i = (i + 1) % l;
            if i == last_i {
                //done
                return Ok(splines.into_iter().map(|maybe| maybe.expect("Should not get a None")).collect());
            }
        }
        Err(ApproxNotFoundError)
    }
}
