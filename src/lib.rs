//!
//! Interpolation method for computation of cubic spline points within
//! the range of a discrete set of known points.
//!
//! [**Demo**](https://emgyrz.github.io/cubic_spline/)
//!
//! # Example
//! ```
//! use cubic_spline::{Points, Point, SplineOpts, TryFrom};
//!
//! let source = vec![(10.0, 200.0), (256.0, 390.0), (512.0, 10.0), (778.0, 200.0)];
//!
//! let opts = SplineOpts::new().tension(0.5);
//!
//! let mut points = Points::try_from(&source).expect("expect valid points but");
//! let result = points.calc_spline(&opts).expect("cant construct spline points");
//!
//! assert_eq!(result.get_ref().len(), 49);
//!
//! let inner_vec: &mut Vec<Point> = points.get_mut();
//! inner_vec.push(Point::new(7.7, 1.3));
//! inner_vec[1].x += 0.79;
//! inner_vec.last_mut().iter_mut().for_each(|mut p| {p.tension = Some(0.7);});
//!
//! points.invert_vertically(400.0);
//!
//! assert_eq!(points.get_ref()[1].y, 10.0);
//!
//! let calculated_points = points
//!   .calc_spline(&opts.num_of_segments(33))
//!   .unwrap();
//!
//! assert_eq!(calculated_points.into_inner().len(), 133);
//!
//! ```
//!
//! For information on how a curve can be constructed and which points to accept,
//! see the appropriate structures.
//!
//! ## Custom points
//!
//! If you already have some points you can implement `From` trait for `Point`
//! struct and pass your points directly.
//!
//! # Example
//! ```
//! use cubic_spline::{SplineOpts, Point, Points};
//!
//! #[derive(Default)]
//! struct MyPoint {
//!   vertical: u8,
//!   horizontal: u8,
//!   color: String,
//! }
//!
//! impl<'a> From<&'a MyPoint> for Point {
//!   fn from(p: &'a MyPoint) -> Self {
//!     Point::new(p.horizontal as f64, p.vertical as f64)
//!   }
//! }
//!
//! let my_points: Vec<MyPoint> = vec![MyPoint::default(),MyPoint::default()];
//! let spline = Points::from(&my_points)
//!   .calc_spline(&SplineOpts::default())
//!   .unwrap();
//!
//! assert_eq!(spline.get_ref().len(), 17);
//!
//! ```
//!

mod calc;
mod convert;
mod impls;
mod opts;
mod points;
mod result;
mod v1;

pub use v1::{Error, Point, Points, Result, TryFrom, TryInto, DEFAULT_APPROX_EQ_PRECISION};

#[cfg(test)]
mod tests;

pub use calc::CalcPoints;
pub use opts::{SplineOpts, SplineOptsBuilder, DEFAULT_SEGMENTS, DEFAULT_TENSION};
pub use points::{GetPoint, SrcPoints};
pub use result::{PushPoint, SplineResult};

///
/// Collection for calculate spline points
#[deprecated(
  since = "1.0.0",
  note = "Please use the `calc_spline` function instead"
)]
pub struct Spline;

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
  #[deprecated(
    since = "1.0.0",
    note = "Please use the `calc_spline` and `Points::try_from_flatten` instead"
  )]
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
  #[deprecated(
    since = "1.0.0",
    note = "Please use the `calc_spline` function instead"
  )]
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
  #[deprecated(since = "1.0.0")]
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
  #[deprecated(since = "1.0.0")]
  pub fn convert_tuples_to_flatten(tuples: &[(f64, f64)]) -> Vec<f64> {
    convert::tuples_to_flatten(tuples)
  }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[allow(clippy::too_many_arguments, non_snake_case)]
#[wasm_bindgen]
pub fn getCurvePoints(
  pts: Vec<f64>,
  num_of_segments: Option<u32>,
  tension: Option<f64>,
  custom_tensions: Option<Vec<f64>>,
  invert_x_with_width: Option<f64>,
  invert_y_with_height: Option<f64>,
  hidden_point_at_start: Option<Vec<f64>>,
  hidden_point_at_end: Option<Vec<f64>>,
) -> Vec<f64> {
  let mut b = SplineOpts::new();

  if let Some(t) = tension {
    b = b.tension(t);
  }

  if let Some(n) = num_of_segments {
    b = b.num_of_segments(n);
  }

  if let Some(s) = hidden_point_at_start {
    if s.len() >= 2 {
      b = b.hidden_point_at_start((s[0], s[1]));
    }
  }

  if let Some(s) = hidden_point_at_end {
    if s.len() >= 2 {
      b = b.hidden_point_at_end((s[0], s[1]));
    }
  }

  let mut pts = Points::try_from_flatten(&pts).unwrap();

  if let Some(w) = invert_x_with_width {
    pts.invert_horizontally(w);
  }

  if let Some(h) = invert_y_with_height {
    pts.invert_vertically(h);
  }

  if let Some(ct) = custom_tensions {
    ct.iter().enumerate().for_each(|(i, t)| {
      if *t > -100.0 {
        pts.get_mut().get_mut(i).iter_mut().for_each(|p| {
          p.tension = Some(*t);
        });
      }
    });
  }

  calc_spline(&pts, &b).into()
}
