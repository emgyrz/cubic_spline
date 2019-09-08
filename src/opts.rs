/// Options for the spline
///
pub struct SplineOpts {
  /// If `true` checks that every x value of point is greater than previous
  ///
  /// default is `false`
  pub disallow_x_stepping_back: bool,

  /// Tension
  ///
  /// default is `0.5_f64`
  pub tension: f64,

  /// Number of calculated points between known points
  ///
  /// default is `16_u32`
  pub num_of_segments: u32,
}

impl Default for SplineOpts {
  /// # Example
  /// ```
  /// use cubic_spline::{Spline, SplineOpts};
  /// let opts = SplineOpts {
  ///   num_of_segments: 10,
  ///   ..Default::default()
  /// };
  ///
  /// let points = vec![10.1, 300.6542, 77.32, 10.42, 375.2, -108.65];
  ///
  /// let spline_points = Spline::from_flatten_points(&points, &opts);
  /// ```
  fn default() -> Self {
    SplineOpts {
      disallow_x_stepping_back: false,
      tension: 0.5,
      num_of_segments: 16,
    }
  }
}
