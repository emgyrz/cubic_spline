//! Interpolation methods for computation of cubic spline points within the range of a discrete set of known points.
//!
//! [**Demo**](https://emgyrz.github.io/cubic_spline/)
//!
//! # Example
//! ```
//! use cubic_spline::{CalcPoints, SplineOpts, SplineResult, SrcPoints};
//!
//! let points = vec![(10.0, 200.0), (256.0, 390.0), (512.0, 10.0), (778.0, 200.0)];
//!
//! let opts = SplineOpts {
//!   num_of_segments: 16,
//!   ..Default::default()
//! };
//!
//! let pts: SrcPoints<(f64, f64)> = SrcPoints::new(&points);
//! let mut result = SplineResult::<(f64, f64)>::new();
//! pts.calc(&opts, &mut result);
//!
//! assert_eq!(result.get().len(), 51);
//!
//! //
//! // Same as:
//! //
//! use cubic_spline::{Spline};
//! let spline_points = Spline::from_tuples(&points, &opts);
//!
//! assert_eq!(spline_points.len(), 51);
//! ```
//!
//!
//!
//! For now source and resulting points may be `Vec<f64> - (vec![x,y,x,y,...])` or `Vec<(f64, f64)> - (vec![(x,y),(x,y), ...])`.
//! For this types of points there are two helper functions `Spline::from_flatten_points` and `Spline::from_tuples`
//!
//!
//! ## Custom points
//!
//! If you allready have some points to avoid unnecessary copying, creating new `Vec` etc. you can implement `GetPoint` trait. And if you need some particular result implement `PushPoint`.
//! # Example
//! ```
//! use cubic_spline::{CalcPoints,SrcPoints,SplineResult,PushPoint,GetPoint,SplineOpts};
//!
//! struct MyPoint {
//!   pub top: f32,
//!   pub left: f32,
//!   pub label: Option<String>,
//! }
//!
//! struct MyResult<T>(T);
//! struct MySrcPoint<T>(T);
//!
//! impl<'a> GetPoint for MySrcPoint<SrcPoints<'a, MyPoint>> {
//!   fn get(&self, index: usize) -> Option<(f64, f64)> {
//!     self.0.pts().get(index).and_then(|p| {
//!       Some((f64::from(p.left), f64::from(p.top)))
//!     })
//!   }
//!   fn len(&self) -> usize {
//!     self.0.pts().len()
//!   }
//! }
//!
//! impl PushPoint for MyResult<SplineResult<MyPoint>> {
//!   fn push_spline_point(&mut self, x: f64, y: f64) {
//!     let calculated_point = MyPoint { top: y as f32, left: x as f32, label: None };
//!     self.0.pts().push(calculated_point);
//!   }
//! }
//!
//! impl<'a> CalcPoints for MySrcPoint<SrcPoints<'a, MyPoint>> {}
//!
//!
//! let points: Vec<MyPoint> = vec![];
//! let pts = MySrcPoint(SrcPoints::new(&points));
//! let mut result = MyResult(SplineResult::default());
//! pts.calc(&SplineOpts::default(), &mut result);
//!
//! ```
//! See [here](https://github.com/emgyrz/cubic_spline/tree/master/src/impls) for implementation example
//!
//!

mod calc;
mod convert;
mod impls;
mod opts;
mod points;
mod result;

#[cfg(test)]
mod tests;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub use calc::CalcPoints;
pub use opts::SplineOpts;
pub use points::{GetPoint, SrcPoints};
pub use result::{PushPoint, SplineResult};

/// Collection for calculate spline points
pub struct Spline();

impl Spline {
  /// Calculates flat vector of points from known points
  ///
  /// Points is vec of `[x, y, x, y, ...]`
  /// # Example
  /// ```
  /// use cubic_spline::{Spline, SplineOpts};
  ///
  /// let opts: SplineOpts = Default::default();
  ///
  /// let points = vec![10.0, 200.0, 256.0, 390.0, 512.0, 10.0, 778.0, 200.0];
  ///
  /// let spline_points = Spline::from_flatten_points(&points, &opts);
  ///
  /// assert_eq!(spline_points.len(), 102);
  /// ```
  pub fn from_flatten_points(points: &[f64], opts: &SplineOpts) -> Vec<f64> {
    let mut result: SplineResult<f64> =
      SplineResult::with_capacity((points.len() / 2) * opts.num_of_segments as usize);

    SrcPoints::new(points).calc(opts, &mut result);
    result.get()
  }

  /// Calculates vector of point tuples from known points
  ///
  /// Points is vec of `[(x, y), (x, y), ...]`
  /// # Example
  /// ```
  /// use cubic_spline::{Spline, SplineOpts};
  ///
  /// let opts: SplineOpts = Default::default();
  ///
  /// let points = vec![(10.0, 200.0), (256.0, 390.0), (512.0, 10.0), (778.0, 200.0)];
  ///
  /// let spline_points = Spline::from_tuples(&points, &opts);
  ///
  /// let (last_x, last_y) = spline_points.last().unwrap();
  ///
  /// assert_eq!(*last_y, 200.0_f64);
  /// ```
  pub fn from_tuples(points: &[(f64, f64)], opts: &SplineOpts) -> Vec<(f64, f64)> {
    let mut result =
      SplineResult::<(f64, f64)>::with_capacity(points.len() * opts.num_of_segments as usize);
    SrcPoints::new(points).calc(opts, &mut result);
    result.get()
  }

  /// Converts flatten points vector to tuples vector.
  ///
  /// # Example
  /// ```
  /// use cubic_spline::{Spline};
  ///
  /// let points = vec![256.0, 390.0, 512.0, 10.0];
  ///
  /// let tuples = Spline::convert_flatten_to_tuples(&points);
  ///
  /// assert_eq!( tuples, vec![(256.0, 390.0), (512.0, 10.0)] );
  /// ```
  pub fn convert_flatten_to_tuples(pts: &[f64]) -> Vec<(f64, f64)> {
    convert::flatten_to_tuples(pts)
  }

  /// Converts tuples vector to flatten.
  ///
  /// # Example
  /// ```
  /// use cubic_spline::{Spline};
  ///
  /// let tuples = vec![(256.0, 390.0), (512.0, 10.0)];
  ///
  /// let points = Spline::convert_tuples_to_flatten(&tuples);
  ///
  /// assert_eq!( points, vec![256.0, 390.0, 512.0, 10.0] );
  /// ```
  pub fn convert_tuples_to_flatten(tuples: &[(f64, f64)]) -> Vec<f64> {
    convert::tuples_to_flatten(tuples)
  }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn getCurvePoints(
  pts: Vec<f64>,
  tension: Option<f64>,
  num_of_segments: Option<u32>,
  disallow_x_stepping_back: Option<bool>,
) -> Vec<f64> {
  let mut opts: SplineOpts = Default::default();

  if let Some(tension) = tension {
    opts.tension = tension;
  }
  if let Some(num_of_segments) = num_of_segments {
    opts.num_of_segments = num_of_segments;
  }
  if let Some(disallow_x_stepping_back) = disallow_x_stepping_back {
    opts.disallow_x_stepping_back = disallow_x_stepping_back;
  }

  Spline::from_flatten_points(&pts, &opts)
}
