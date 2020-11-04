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
//! let opts = SplineOpts::new()
//!   .tension(0.5);
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

#[cfg(test)]
mod tests;

mod calc;
mod err;
mod opts;
mod points;
mod points_iter;
mod tfti;

#[cfg(target_arch = "wasm32")]
mod wasm;

pub use calc::calc_spline;

pub use err::{Error, Result};
pub use opts::{SplineOpts, DEFAULT_SEGMENTS, DEFAULT_TENSION};
pub use points::{Point, Points, DEFAULT_APPROX_EQ_PRECISION};
pub use tfti::{TryFrom, TryInto};

#[cfg(target_arch = "wasm32")]
pub use wasm::getCurvePoints;
