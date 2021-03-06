<a id="english">

# cucoqu — Cubic ← Conic ← Quadratic
[日本語版を読む](#japanese)

<img src="blob/cucoqu.png" width="250">

© 2022 Fredrick R. Brennan, MFEK Authors, Skia Authors, and GPT-3<sup>[huh?](GPT-3.md)</sup>

**cucoqu** is a Rust library for converting between different types of Bézier splines. Currently it only supports the types most commonly used in type design and drawing curves: **cu**bic Bézier curves, **qu**adratic Bézier curves, and one type of rational Bézier curve: **co**nic Bézier curves.

This library replaces a lot of calls into Skia's C++ code and conversion back and forth from its point types in MFEK.

## Todo

* arbitrarily raise degree of an n-degree bezier segment 

## API
### Types
#### Cubic
```rust
// Bézier segments
/// A cubic [`Bezier` segment][cubic-bezier].
///
/// [cubic-bezier]: https://en.wikipedia.org/wiki/B%C3%A9zier_curve#Higher-order_B%C3%A9zier_curves
pub type Cubic = [Point; 4];
/// A cubic spline.
pub type CubicSpline = Vec<Cubic>;
```
#### Conic
```rust
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
    pub weight: f32
}
```
#### Quad
```rust
/// A quadratic [`Bezier` segment][quad-bezier].
///
/// [quad-bezier]: https://en.wikipedia.org/wiki/B%C3%A9zier_curve#Quadratic_curves
pub type Quad = [Point; 3];
/// A [quadratic spline][quad-spline].
///
/// [quad-spline]: https://en.wikipedia.org/wiki/B-spline#Quadratic_splines
pub type QuadSpline = Vec<Quad>;
```
### Conversions
#### cu2qu
```rust
/// Convert a cubic Bézier curve to a quadratic spline segment.
pub trait CurveToQuadratic {
    fn curve_to_quadratic(&self, max_err: f32) -> Result<QuadSpline, ApproxNotFoundError>;
}

/// Convert a vector of cubic Bézier curves to a vector of quadratic spline segments.
pub trait CurvesToQuadratic {
    fn curves_to_quadratic(&self, max_errors: Vec<f32>) -> Result<Vec<QuadSpline>, ApproxNotFoundError>;
}

impl CurveToQuadratic for Cubic { … }
impl CurvesToQuadratic for Vec<Cubic> { … }
```
#### co2qu
```rust
/// Convert a “conic” (rational quadratic) Bézier curve to N quadratic spline segments.
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
```
#### qu2cu
```rust
/// Trait for exact cubic Bézier curve generation from quadratic.
pub trait QuadToCubic<QCO: Default, const N: usize> {
    fn quad_to_cubic(self) -> [QCO; N];
}
```
<a id="japanese">

# cucoqu（ク・コ・キュ） — 三次←円錐←二次 （三⇒曲⇐円⇒線⇐二）
[Read in English](#english)

<img src="blob/cucoqu.png" width="250">

© 2022 フレッド・R・ブレンナン、MFEK著者達、Skia著者達、そしてＧＰＴ３型<sup>[なに？](GPT-3（日本語版）.md)</sup>

**cucoqu**（読み方：ク・コ・キュ）は、異なるタイプのベジェ自由曲線を変換するための Rust（ラスト）ライブラリです。
現在、タイプデザインや曲線の描画で最も一般的に使用されるタイプのみをサポートしています：三次ベジエ曲線、二次ベジエ曲線、そして円錐線（二次有理ベジェ）です。

このライブラリは、MFEK内のスキアのC＋＋コードに呼び出しを多く置き換え、そのポイントタイプにおける往復の変換を置き換えます。

## やる可きリスト

* 任意の _n_ 次ベジエ曲線の多項式を上げる

## API
### 型一覧
#### Cubic (cu)
`self[0]` から `self[3]` までを貫く三次ベジェ曲線（セグメント）。
`self[1..=2]` は制御点。
つまり`Cubic`の型とは、4点の凸包を含む。
結果として`CubicSpline`の型とは、三次ベジェ曲線の配列である。

```rust
pub type Cubic = [Point; 4];
pub type CubicSpline = Vec<Cubic>;
```
#### Conic (co)
始点 `start` と終点 `end` と制御点（及び頂点）`control` と重み `weight`（_w_）によって定義される円錐曲線。重みは非負の数で、デフォルトは1である。円錐曲線は、重みと制御点、始点、終点の相対的な位置によって、楕円、円、放物線、または双曲線と呼ばれる。

**👉&#xfe0e;_w_ について**

_w_ とは円錐曲線の重さである。

_w_ ＝１ならば、円錐曲線は**放物線部**である。<br/>
_w_ が１より小さい場合、　**楕円曲線部**である。<br/>
_w_ が１より大きい場合、　**双曲線部**である。
```rust
#[derive(Clone, Default)]
pub struct Conic {
    pub start: Point,
    pub end: Point,
    pub control: Point,
    pub weight: f32
}
```
#### Quad (qu)
`self[0]` から `self[2]` までを貫く三次ベジェ曲線（セグメント）。
`self[1]` は頂点。
つまり`Quad`の型とは、3点の凸包を含んでいるから凸包は必ず三角形になる。
結果として`QuadSpline`の型とは、二次ベジェ曲線の配列である。
```rust
pub type Quad = [Point; 3];
pub type QuadSpline = Vec<Quad>;
```
### Conversions
#### cu2qu
三次ベジエ曲線を二次曲線に変換。
```rust
pub trait CurveToQuadratic {
    fn curve_to_quadratic(&self, max_err: f32) -> Result<QuadSpline, ApproxNotFoundError>;
}

pub trait CurvesToQuadratic {
    fn curves_to_quadratic(&self, max_errors: Vec<f32>) -> Result<Vec<QuadSpline>, ApproxNotFoundError>;
}

impl CurveToQuadratic for Cubic { … }
impl CurvesToQuadratic for Vec<Cubic> { … }
```
#### co2qu
二次有理ベジェは二次ベジエ曲線らに変換。
```rust
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
```
#### qu2cu
二次ベジエ曲線を三次曲線に変換。
```rust
pub trait QuadToCubic<QCO: Default, const N: usize> {
    fn quad_to_cubic(self) -> [QCO; N];
}
```
