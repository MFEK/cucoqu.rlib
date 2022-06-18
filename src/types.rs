use derive_more::{Add, Div, Mul, Sub};
use glifparser::{
    IntegerOrFloat::{self, Float},
    PointData, PointLike,
};

/// Convenience point constructor.
#[macro_export]
macro_rules! point {
    [$($sub:tt)*] => [
        Point::from_xy($($sub)*)
    ]
}

// Bézier segments
/// A cubic [`Bezier` segment][cubic-bezier].
///
/// [cubic-bezier]: https://en.wikipedia.org/wiki/B%C3%A9zier_curve#Higher-order_B%C3%A9zier_curves
pub type Cubic = [Point; 4];
/// A quadratic [`Bezier` segment][quad-bezier].
///
/// [quad-bezier]: https://en.wikipedia.org/wiki/B%C3%A9zier_curve#Quadratic_curves
pub type Quad = [Point; 3];
// Bézier paths
/// A [cubic spline][cubic-spline].
///
/// [cubic-spline]: https://en.wikipedia.org/wiki/Cubic_Hermite_spline
pub type CubicSpline = Vec<Cubic>;
/// A [quadratic spline][quad-spline].
///
/// [quad-spline]: https://en.wikipedia.org/wiki/B-spline#Quadratic_splines
pub type QuadSpline = Vec<Quad>;

pub(crate) type Float2 =
    nalgebra::Matrix<f32, nalgebra::Const<2_usize>, nalgebra::Const<1_usize>, nalgebra::ArrayStorage<f32, 2_usize, 1_usize>>;

/// A two-dimensional point (with x and y coordinates).
#[derive(
    derive_more::Constructor,
    derive_more::From,
    derive_more::Deref,
    derive_more::DerefMut,
    Add,
    Mul,
    Sub,
    Div,
    Copy,
    Clone,
    Default,
    Debug,
    PartialEq,
)]
pub struct Point(pub Float2);

impl PointData for Point {}

impl Point {
    pub const fn from_xy(x: f32, y: f32) -> Self {
        Point(nalgebra::SMatrix::from_array_storage(nalgebra::ArrayStorage::<f32, 2, 1>([[x, y]])))
    }
}

/// The `PointLike` trait from the [`glifparser`] crate, but implemented for the `Point` structure.
///
/// This allows us to use the `Point` structure in a `glifparser`-compatible way.
impl PointLike for Point {
    fn x(&self) -> IntegerOrFloat {
        Float(self.0[0])
    }
    fn y(&self) -> IntegerOrFloat {
        Float(self.0[1])
    }
    fn set_x(&mut self, x: IntegerOrFloat) {
        self.0[0] = x.into();
    }
    fn set_y(&mut self, y: IntegerOrFloat) {
        self.0[1] = y.into();
    }
}

/// The trait for types which implement the calculation of their [derivative coefficients][der-coeffs].
///
/// [der-coeffs]: https://wikipedia.org/wiki/B%C3%A9zier_curve#Higher-order_B%C3%A9zier_curves
pub trait DerivativeCoefficients<const N: usize> {
    fn deriv_coeff(&self) -> [f32; N];
}

use std::ops::{Add, Mul};

impl Mul<Point> for f32 {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        Point(self * rhs.0)
    }
}
impl Add<Point> for Float2 {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point(self + rhs.0)
    }
}
