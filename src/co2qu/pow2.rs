use super::Conic;
use super::consts::MAX_QUAD_POW2;

macro_rules! as_quad_error_setup {
    ($conic:expr, $s:ident, $c:ident, $e:ident, $w:ident, $a:ident, $k:ident, $x:ident, $y:ident) => {
        use ::glifparser::PointLike;
        $s = &$conic.start;
        $c = &$conic.control;
        $e = &$conic.end;
        $w = $conic.weight;
        $a = ($w - 1f32);
        $k = $a / (4f32 * (2f32 + $a));
        $x = f32::from($k * ($s.x() - 2f32 * $c.x() + $e.x()));
        $y = f32::from($k * ($s.y() - 2f32 * $c.y() + $e.y()));
    }
}

/// Trait for calculating whether or not a value is below the user-provided quadratic tolerance.
pub trait BelowQuadTolerance {
    fn quad_error(&self) -> [f32; 2];
    fn below_quad_tolerance(&self, tol: f32) -> bool;
}

/// Compute the number of subdivisions needed to approximate the conic with a quadratic.
pub trait QuadPowerOf2 {
    /// Compute the number of subdivisions needed to approximate the conic with a quadratic.
    fn quad_pow2(&self, tol: f32) -> usize;
}

/// An internal trait used to determine whether a conic is below a given quadratic tolerance.
impl BelowQuadTolerance for Conic {
    #[inline]
    fn quad_error(&self) -> [f32; 2] {
        let (s, c, e, k, w, a, x, y);
        as_quad_error_setup!(self, s, c, e, k, w, a, x, y);
        [x, y]
    }

    fn below_quad_tolerance(&self, tol: f32) -> bool {
        let [x, y] = self.quad_error();
        (x * x + y * y) <= tol * tol
    }
}

impl QuadPowerOf2 for Conic {
    fn quad_pow2(&self, tol: f32) -> usize {
        let (s, c, e, k, w, a, x, y);
        as_quad_error_setup!(self, s, c, e, k, w, a, x, y);
        let mut error: f32 = (x * x + y * y).sqrt();
        let mut pow2 = 0;
        if error.is_nan() {
            return 0;
        }
        for i in 0..MAX_QUAD_POW2 {
            pow2 = i;
            if error <= tol {
                break;
            }
            error *= 0.25;
        }
        pow2
    }
}
