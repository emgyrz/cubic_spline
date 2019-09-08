mod calc;
mod convert;
mod from_tuples;
mod opts;
mod points;

#[cfg(test)]
mod test;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub use opts::SplineOpts;

///! Interpolation methods for computation of cubic spline points
///! within the range of a discrete set of known points.

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
    let pts = convert::flatten_to_tuples(points);
    let tuples = from_tuples::get_curve_points(&pts, opts);
    convert::tuples_to_flatten(&tuples)
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
    from_tuples::get_curve_points(points, opts)
  }

  /// Converts flatten points vector to tuples vector.
  ///
  /// # Example
  /// ```
  /// use cubic_spline::{Spline};
  ///
  /// let points = vec![256.0, 390.0, 512.0, 10.0];
  ///
  /// let tuples = Spline::convert_flatten_to_tuples(&point);
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
