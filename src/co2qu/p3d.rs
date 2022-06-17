use glifparser::PointLike;

use crate::p3d::{Point3, Point3Like};
use crate::{Point, point};

#[inline]
pub fn ratquad_map<PL: PointLike>(src: &[PL; 3], w: f32) -> [Point3; 3] {
    [
        Point3::new(src[0].x(), src[0].y() * 1f32, 1f32),
        Point3::new(src[1].x() * w, src[1].y() * w, w),
        Point3::new(src[2].x(), src[2].y() * 1f32, 1f32),
    ]
}

#[inline]
pub fn interp(src: &[Point3; 3], t: f32) -> [Point3; 3] {
    let (src0, src1, src2) = (src[0].as_linalg(), src[1].as_linalg(), src[2].as_linalg());
    let ab = src0.lerp(&src1, t);
    let bc = src1.lerp(&src2, t);
    let abbc = ab.lerp(&bc, t);
    [Point3::from_linalg(ab), Point3::from_linalg(abbc), Point3::from_linalg(bc)]
}

impl Into<super::Point> for Point3 {
    // from Skia project_down
    // static SkPoint project_down(const SkPoint3& src) { return {src.fX / src.fZ, src.fY / src.fZ}; }
    fn into(self) -> super::Point {
        point![f32::from(self.x() / self.z()), f32::from(self.y() / self.z())]
    }
}
