/// Wrapper for source points. Can be used for implementing `GetPoint` and `CalcPoints` traits
pub struct SrcPoints<'a, T> {
  pts: &'a [T],
}

pub(crate) type PointsToCalc<T> = ((T, T), (T, T), (T, T), (T, T));

impl<'a, T> SrcPoints<'a, T> {
  /// Constructs a new `SrcPoints` with ref to source points slice
  /// # Example
  /// ```
  /// use cubic_spline::SrcPoints;
  /// let points = vec![(10.0, 200.0), (256.0, 390.0), (512.0, 10.0), (778.0, 200.0)];
  /// let wrp = SrcPoints::new(&points);
  /// ```
  pub fn new(pts: &'a [T]) -> Self {
    SrcPoints { pts }
  }

  /// Returns referense of inner slice
  pub fn pts(&self) -> &'a [T] {
    self.pts
  }
}

/// Trait for get points needed for calculating
/// If you implement this trait for your own struct, you just need write `get` and `len` methods
/// # Example
/// ```
/// use cubic_spline::{GetPoint};
///
/// // Example for points represents as `Vec<x,y,x,y...>`
/// struct MyPoints {
///   list: Vec<f64>
/// };
///
/// impl GetPoint for MyPoints {
///   fn get(&self, index: usize) -> Option<(f64, f64)> {
///     let x_ind = index * 2;
///     let x = self.list.get(x_ind);
///     let y = self.list.get(x_ind + 1);
///     x.and(y).map(|y| (*x.unwrap(), *y))
///   }
///   fn len(&self) -> usize {
///     self.list.len() / 2
///   }
/// }
/// ```
#[allow(clippy::len_without_is_empty)]
pub trait GetPoint {
  /// Returns length of points. As usual if your point combined to struct, tuple etc. (for example, `(0, 108)` or `Point { top: 108, left: 0 }`) it will be a `Vec<PointType>.len()`
  fn len(&self) -> usize;

  /// Returns point by index. Coordinates must be presented as f64
  fn get(&self, index: usize) -> Option<(f64, f64)>;

  fn last(&self) -> Option<(f64, f64)> {
    self.get(self.len() - 1)
  }
  fn points_to_calc(&self, index: usize) -> Option<PointsToCalc<f64>> {
    let current = self.get(index)?;
    let next = self.get(index + 1)?;

    let prev = if index == 0 {
      current
    } else {
      self.get(index - 1).unwrap_or_else(|| current)
    };

    let next2 = self.get(index + 2).unwrap_or_else(|| next);

    Some((prev, current, next, next2))
  }
}
