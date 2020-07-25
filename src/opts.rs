/// Options for the spline
///
pub struct SplineOpts {
  /// Tension
  ///
  /// default is `0.5_f64`
  pub tension: f64,

  /// Number of calculated points between known points
  ///
  /// default is `16_u32`
  pub num_of_segments: u32,

  /// If set to Some(canvas_width) generated points will be inverted by X-axis.
  ///
  pub invert_x_with_width: Option<u32>,

  /// If set to Some(canvas_height) generated points will be inverted by Y-axis.
  ///
  pub invert_y_with_height: Option<u32>,

  /// A point that will not be drawn,
  /// but the beginning of the graph will bend as if it is there.
  ///
  pub hidden_point_at_start: Option<(f64, f64)>,

  /// A point that will not be drawn,
  /// but the end of the graph will bend as if it is there.
  ///
  pub hidden_point_at_end: Option<(f64, f64)>,
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
      tension: 0.5,
      num_of_segments: 16,
      invert_x_with_width: None,
      invert_y_with_height: None,
      hidden_point_at_start: None,
      hidden_point_at_end: None,
    }
  }
}

pub struct SplineOptsBuilder {
  opts: SplineOpts,
}

/// Builder pattern implementation for `SplineOpts`
impl SplineOptsBuilder {
  /// Creates new one
  pub fn new() -> Self {
    SplineOptsBuilder::default()
  }

  /// Returns new builded spline opts
  pub fn take(self) -> SplineOpts {
    self.opts
  }

  /// Sets tension
  pub fn tension(mut self, val: f64) -> Self {
    self.opts.tension = val;
    self
  }

  /// Sets num_of_segments
  pub fn num_of_segments(mut self, val: u32) -> Self {
    self.opts.num_of_segments = val;
    self
  }

  /// Sets invert_x_with_width
  pub fn invert_x_with_width(mut self, val: u32) -> Self {
    self.opts.invert_x_with_width = Some(val);
    self
  }

  /// Sets invert_y_with_height
  pub fn invert_y_with_height(mut self, val: u32) -> Self {
    self.opts.invert_y_with_height = Some(val);
    self
  }

  /// Sets hidden_point_at_start
  pub fn hidden_point_at_start(mut self, val: (f64, f64)) -> Self {
    self.opts.hidden_point_at_start = Some(val);
    self
  }

  /// Sets hidden_point_at_end
  pub fn hidden_point_at_end(mut self, val: (f64, f64)) -> Self {
    self.opts.hidden_point_at_end = Some(val);
    self
  }
}

impl Default for SplineOptsBuilder {
  fn default() -> Self {
    SplineOptsBuilder {
      opts: SplineOpts::default(),
    }
  }
}
