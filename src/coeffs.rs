use crate::types::Point;
use crate::point;

/// A pub trait for types of curves we can transform to and from coefficients
pub trait Coefficient {
    /// @param t where on the curve we want a point from
    /// @return a point of type Point
    fn eval(&self, t: Point) -> Point;
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// A conic section curve with a real quadratic coeff numerator and a real quadratic coeff denominator
pub struct ConicCoeff {
    /// The quadratic coefficient form of the point
    numer: QuadCoeff,
    /// The quadratic coefficient form of the "weight" of the curve
    denom: QuadCoeff,
}

impl Coefficient for ConicCoeff {
    fn eval(&self, t: Point) -> Point {
        let numer = self.numer.eval(t);
        let denom = self.denom.eval(t);
        Point(numer.component_div(&denom))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// A real cubic Bezier curve
pub struct CubicCoeff {
    a: Point,
    b: Point,
    c: Point,
    d: Point,
}

impl Coefficient for CubicCoeff {
    fn eval(&self, t: Point) -> Point {
        ((self.a.component_mul(&t) + self.b).component_mul(&t) + self.c).component_mul(&t) + self.d
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// A real quadratic Bezier curve
pub struct QuadCoeff {
    a: Point,
    b: Point,
    c: Point,
}

impl Coefficient for QuadCoeff {
    fn eval(&self, t: Point) -> Point {
        (self.a.component_mul(&t) + self.b).component_mul(&t) + self.c
    }
}

/// Trait for converting a segment of a curve to a coefficient type
pub trait FromSegment<const SEGMENT_LEN: usize> {
    /// @param segment the segment of the curve to convert
    /// @return a coefficient type
    fn from_segment(segment: [Point; SEGMENT_LEN]) -> Self;
}

pub trait FromWeightedSegment<const SEGMENT_LEN: usize, const WEIGHT_LEN: usize> {
    /// @param segment the segment of the curve to convert
    /// @param w the weight of the curve
    /// @return a coefficient type
    fn from_segment(segment: [Point; SEGMENT_LEN], w: [f32; WEIGHT_LEN]) -> Self;
}

impl FromWeightedSegment<3, 1> for ConicCoeff {
    fn from_segment(segment: [Point; 3], w: [f32; 1]) -> Self {
        let p0 = segment[0];
        let p1 = segment[1];
        let p2 = segment[2];
        let w = w[0];

        let p1w = p1 * w;
        let numer_c = p0;
        let numer_a = p2 - (p1w * 2f32) + p0;
        let numer_b = (p1w - p0) * 2f32;

        let denom_c = point![1f32, 1f32];
        let denom_b = (point![w, w] - denom_c) * 2f32;
        let denom_a = point![0f32, 0f32] - denom_b;

        ConicCoeff {
            numer: QuadCoeff {
                a: numer_a,
                b: numer_b,
                c: numer_c,
            },
            denom: QuadCoeff {
                a: denom_a,
                b: denom_b,
                c: denom_c,
            },
        }
    }
}

impl FromSegment<4> for CubicCoeff {
    fn from_segment(segment: [Point; 4]) -> Self {
        let p0 = segment[0];
        let p1 = segment[1];
        let p2 = segment[2];
        let p3 = segment[3];

        CubicCoeff {
            a: p3 + 3f32 * (p1 - p2) - p0,
            b: 3f32 * (p2 - (p1 * 2f32) + p0),
            c: 3f32 * (p1 - p0),
            d: p0,
        }
    }
}

impl FromSegment<3> for QuadCoeff {
    fn from_segment(segment: [Point; 3]) -> Self {
        let p0 = segment[0];
        let p1 = segment[1];
        let p2 = segment[2];

        QuadCoeff {
            a: p2 - (p1 * 2f32) + p0,
            b: (p1 - p0) * 2f32,
            c: p0,
        }
    }
}

/// Trait for converting a coefficient type to segment of a curve
pub trait ToSegment<const SEGMENT_LEN: usize> {
    fn to_segment(&self) -> [Point; SEGMENT_LEN];
}

impl ToSegment<4> for CubicCoeff {
    fn to_segment(&self) -> [Point; 4] {
        let p0 = self.d;
        let p1 = (self.c / 3f32) + self.d;
        let p2 = (self.b + self.c) / 3f32 + p1;
        let p3 = self.a + self.d + self.c + self.b;
        [p0, p1, p2, p3]
    }
}

impl ToSegment<3> for QuadCoeff {
    fn to_segment(&self) -> [Point; 3] {
        let p0 = self.c;
        let p1 = (self.b / 2f32) + self.c;
        let p2 = self.a + self.b + self.c;
        [p0, p1, p2]
    }
}

impl ToSegment<3> for ConicCoeff {
    fn to_segment(&self) -> [Point; 3] {
        let p0 = self.numer.c.component_div(&self.denom.c);
        let p1 = (self.numer.b.component_div(&self.denom.c) + self.numer.c).component_div(&self.denom.c);
        let p2 = (self.numer.a + self.numer.b + self.numer.c).component_div(&self.denom.c);
        [Point(p0), Point(p1), Point(p2)]
    }
}


// TODO: Figure out if this is a thing. GPT-3 came up with it, "cubic conics" may not even be useful.

#[derive(Debug, Clone, Copy, PartialEq)]
/// A conic section curve with a real cubic coeff numerator and a real cubic coeff denominator
pub struct CubicConicCoeff {
    numer: CubicCoeff,
    denom: CubicCoeff,
}

impl Coefficient for CubicConicCoeff {
    fn eval(&self, t: Point) -> Point {
        let numer = self.numer.eval(t);
        let denom = self.denom.eval(t);
        Point(numer.component_div(&denom))
    }
}

impl FromWeightedSegment<4, 2> for CubicConicCoeff {
    fn from_segment(segment: [Point; 4], w: [f32; 2]) -> Self {
        let [p0, p1, p2, p3] = segment;
        let [w0, w1] = w;

        let p1w = p1 * w0;
        let p2w = p2 * w1;

        let numer_a = p3 + 3f32 * (p1w - p2w) - p0;
        let numer_b = 3f32 * (p2w - (p1w * 2f32) + p0);
        let numer_c = 3f32 * (p1w - p0);
        let numer_d = p0;

        let denom_a = point![0f32, 0f32];
        let denom_b = (point![w0, w1] - denom_a) * 3f32;
        let denom_c = (point![0f32, 0f32] - denom_b) * 3f32;
        let denom_d = point![1f32, 1f32];

        CubicConicCoeff {
            numer: CubicCoeff {
                a: numer_a,
                b: numer_b,
                c: numer_c,
                d: numer_d,
            },
            denom: CubicCoeff {
                a: denom_a,
                b: denom_b,
                c: denom_c,
                d: denom_d,
            },
        }
    }
}
