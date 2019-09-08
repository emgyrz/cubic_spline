/// Wrapper for resulting vector of "points".
/// "Points" type may be any which implements `PushPoint` trait
pub struct SplineResult<T> {
  pts: Vec<T>,
}

impl<T> SplineResult<T> {
  /// Constructs a new `SplineResult<T>` with empty `Vec<T>`
  ///
  /// # Example
  /// ```
  /// use cubic_spline::{SplineResult};
  ///
  /// let mut result = SplineResult::<f64>::new();
  ///
  /// assert!(result.pts().is_empty());
  /// ```
  pub fn new() -> Self {
    SplineResult { pts: Vec::new() }
  }

  /// Constructs a new `SplineResult<T>` with empty `Vec<T>` with the specified capacity.
  ///
  /// # Example
  /// ```
  /// use cubic_spline::{SplineResult};
  ///
  /// let points = vec![(10.0, 200.0), (256.0, 390.0), (512.0, 10.0), (778.0, 200.0)];
  /// let num_of_segments = 23;
  ///
  /// let mut result: SplineResult<(f64,f64)> = SplineResult::with_capacity(points.len() * num_of_segments);
  ///
  /// assert!(result.pts().is_empty());
  /// ```
  pub fn with_capacity(capacity: usize) -> Self {
    SplineResult {
      pts: Vec::with_capacity(capacity),
    }
  }

  /// Moves the resulting point out of the `SplineResult`
  pub fn get(self) -> Vec<T> {
    self.pts
  }

  /// Get mutable referense to inner `Vec<T>`
  pub fn pts(&mut self) -> &mut Vec<T> {
    self.pts.as_mut()
  }
}

pub trait PushPoint {
  fn push_spline_point(&mut self, x: f64, y: f64);
}

impl<T> Default for SplineResult<T> {
  fn default() -> Self {
    SplineResult::new()
  }
}
