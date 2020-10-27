use crate::Point;

///
/// Default tension of curve between passed points.
/// Will be used if not specified in [`SplineOpts`].
///
/// [`SplineOpts`]: struct.SplineOpts.html
pub const DEFAULT_TENSION: f64 = 0.5;

///
/// Default number of segments.
/// Will be used if not specified in [`SplineOpts`].
///
/// [`SplineOpts`]: struct.SplineOpts.html
pub const DEFAULT_SEGMENTS: u32 = 16;

///
/// A list of options indicating how the spline should be calculated
///
/// ```
/// use cubic_spline::SplineOpts;
///
/// let o1 = SplineOpts::default();
///
/// let o2 = SplineOpts::new();
///
/// let o3 = SplineOpts::new()
///   .tension(0.5)
///   .num_of_segments(16);
///
/// ```
///
/// #### Deprecation warning
/// All properties will soon become private. For set options use appropriate methods instead.
///
#[derive(Debug)] // TODO : del debug
pub struct SplineOpts {
  ///
  /// Tension between passed points.
  /// Sets the bending strength of the curve.
  /// If not specified [`DEFAULT_TENSION`] will be used.
  ///
  /// [`DEFAULT_TENSION`]: constant.DEFAULT_TENSION.html
  pub tension: f64,

  ///
  /// The number of points to be calculated between each two known points.
  /// If not specified [`DEFAULT_SEGMENTS`] will be used.
  ///
  /// [`DEFAULT_SEGMENTS`]: constant.DEFAULT_SEGMENTS.html
  pub num_of_segments: u32,

  ///
  /// If set to Some(canvas_width) generated points will be inverted by X-axis.
  #[deprecated(
    since = "1.0.0",
    note = "Instead of this please invert points before passing its in calc method with `Points::invert_horizontally(W)`"
  )]
  pub invert_x_with_width: Option<u32>,

  ///
  /// If set to Some(canvas_height) generated points will be inverted by Y-axis.
  #[deprecated(
    since = "1.0.0",
    note = "Instead of this please invert points before passing its in calc method with `Points::invert_vertically(H)`"
  )]
  pub invert_y_with_height: Option<u32>,

  ///
  /// A point that will not be drawn,
  /// but the beginning of the graph will bend as if it is there.
  pub hidden_point_at_start: Option<Point>,

  ///
  /// A point that will not be drawn,
  /// but the end of the graph will bend as if it is there.
  pub hidden_point_at_end: Option<Point>,
}

impl SplineOpts {
  ///
  /// Creates new one with defaults.
  pub fn new() -> Self {
    SplineOpts::default()
  }

  ///
  /// Sets tension.
  pub fn tension(mut self, val: f64) -> Self {
    self.tension = val;
    self
  }

  ///
  /// Sets num_of_segments.
  pub fn num_of_segments(mut self, val: u32) -> Self {
    self.num_of_segments = val;
    self
  }

  ///
  /// Sets hidden_point_at_start.
  pub fn hidden_point_at_start<T: Into<Point>>(mut self, val: T) -> Self {
    self.hidden_point_at_start = Some(val.into());
    self
  }

  ///
  /// Sets hidden_point_at_end.
  pub fn hidden_point_at_end<T: Into<Point>>(mut self, val: T) -> Self {
    self.hidden_point_at_end = Some(val.into());
    self
  }
}

impl Default for SplineOpts {
  ///
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
      tension: DEFAULT_TENSION,
      num_of_segments: DEFAULT_SEGMENTS,
      invert_x_with_width: None,
      invert_y_with_height: None,
      hidden_point_at_start: None,
      hidden_point_at_end: None,
    }
  }
}

#[deprecated(
  since = "1.0.0",
  note = "All its functionality has been moved to the `SplineOpts`"
)]
pub struct SplineOptsBuilder {
  opts: SplineOpts,
}

///
/// Builder for [`SplineOpts`]
///
/// [`SplineOpts`]: struct.SplineOpts.html
impl SplineOptsBuilder {
  ///
  /// Creates new one
  pub fn new() -> Self {
    SplineOptsBuilder::default()
  }

  // #[deprecated(
  //   since = "1.0.0",
  //   note = "Please use the `build` function instead"
  // )]

  ///
  /// Returns new built spline opts
  pub fn take(self) -> SplineOpts {
    self.opts
  }

  ///
  /// Sets tension
  pub fn tension(mut self, val: f64) -> Self {
    self.opts.tension = val;
    self
  }

  ///
  /// Sets num_of_segments
  pub fn num_of_segments(mut self, val: u32) -> Self {
    self.opts.num_of_segments = val;
    self
  }

  ///
  /// Sets invert_x_with_width
  pub fn invert_x_with_width(mut self, val: u32) -> Self {
    self.opts.invert_x_with_width = Some(val);
    self
  }

  ///
  /// Sets invert_y_with_height
  pub fn invert_y_with_height(mut self, val: u32) -> Self {
    self.opts.invert_y_with_height = Some(val);
    self
  }

  ///
  /// Sets hidden_point_at_start
  pub fn hidden_point_at_start<T: Into<Point>>(mut self, val: T) -> Self {
    self.opts.hidden_point_at_start = Some(val.into());
    self
  }

  ///
  /// Sets hidden_point_at_end
  pub fn hidden_point_at_end<T: Into<Point>>(mut self, val: T) -> Self {
    self.opts.hidden_point_at_end = Some(val.into());
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
