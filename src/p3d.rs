use glifparser::{
    IntegerOrFloat::{self, Float},
    PointData, PointLike,
};
use nalgebra::{vector, ArrayStorage, Const, Matrix};

/// A 3D point type, containing X, Y, and Z coordinates.
#[derive(Copy, Clone, Debug, Default)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl PointLike for Point3 {
    fn x(&self) -> IntegerOrFloat {
        Float(self.x)
    }
    fn y(&self) -> IntegerOrFloat {
        Float(self.y)
    }
    fn set_x(&mut self, x: IntegerOrFloat) {
        self.x = x.into();
    }
    fn set_y(&mut self, y: IntegerOrFloat) {
        self.y = y.into();
    }
}

pub trait Point3Like: PointLike {
    /// Fetch the Z coordinate.
    fn z(&self) -> IntegerOrFloat;
    fn set_z(&mut self, z: IntegerOrFloat);
}

impl Point3Like for Point3 {
    fn z(&self) -> IntegerOrFloat {
        Float(self.z)
    }
    fn set_z(&mut self, z: IntegerOrFloat) {
        self.z = z.into();
    }
}

type Point3Matrix = Matrix<f32, Const<3_usize>, Const<1_usize>, ArrayStorage<f32, 3_usize, 1_usize>>;

impl Point3 {
    /// Construct a new point.
    pub fn new(x: impl Into<f32>, y: impl Into<f32>, z: impl Into<f32>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
    /// Convert the Point3 to a matrix suitable for linear algebra.
    pub(crate) fn as_linalg(&self) -> Point3Matrix {
        vector![self.x().into(), self.y().into(), self.z().into()]
    }
    /// Construct a Point3 from a matrix suitable for linear algebra.
    pub(crate) fn from_linalg(f: Point3Matrix) -> Self {
        Self { x: f[0], y: f[1], z: f[2] }
    }
}

impl PointData for Point3 {}
