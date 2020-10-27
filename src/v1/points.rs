use crate::v1::calc_spline;
use crate::{Error, Result, SplineOpts, TryFrom};

/// Default precision used for point comparison in [`approx_eq`] method.
///
/// [`approx_eq`]: struct.Point.html#method.approx_eq
pub const DEFAULT_APPROX_EQ_PRECISION: f64 = 1e-6;

///
/// The point in 2d coordinate system.
///
#[derive(Clone, Default, Debug)]
pub struct Point {
  ///
  /// x-axis point value.
  pub x: f64,

  ///
  /// y-axis point value.
  pub y: f64,

  ///
  /// Optional tension of the curve between this point and the next point.
  pub tension: Option<f64>,
}

///
/// Wrapper for your source points.
/// Prepares and validates points before calculating spline.
/// Create it with [`try_from`]`/`[`try_into`].
/// Or if you are very confident in the validity of your points use usual `From/Into` traits,
/// which will return an empty `Vec` on error.
///
/// # Example
/// ```
/// use cubic_spline::{Points, TryFrom};
///
/// let src1 = vec![[1.2, 3.3], [122.2, 333.3]];
/// let src2 = [[1.2, 3.3], [122.2, 333.3]];
/// let src3 = [(1.2, 3.3), (122.2, 333.3)];
/// let src4 = [1.2, 3.3, 122.2, 333.3];
///
/// assert!(Points::try_from(&src1).is_ok());
/// assert!(Points::try_from(&src2).is_ok());
/// assert_eq!(Points::from(&src3).get_ref().len(), 2);
///
/// let points1 = Points::try_from_flatten(&src4).unwrap();
/// let points2: Points = src1.into();
/// let first1 = points1.get_ref().first().unwrap();
///
/// assert!(first1.approx_eq(&points2.into_inner()[0]));
///
/// ```
///
/// [`try_from`]: trait.TryFrom.html#tymethod.try_from
/// [`try_into`]: trait.TryInto.html#tymethod.try_into
///
#[derive(Clone, Debug)]
pub struct Points(Vec<Point>);

//
//
//
//
//////////////////////////////////////////////////////
// POINT OWN IMPL
//////////////////////////////////////////////////////
impl Point {
  ///
  /// Creates new point. You may prefer use `From`/`Into` implementations for this.
  ///
  pub fn new(x: f64, y: f64) -> Self {
    Point {
      x,
      y,
      tension: None,
    }
  }

  ///
  /// Creates new point with tension of the curve between it and the next point.
  /// If points creates with `::new` the tension from [`SplineOpts`] will be used.
  ///
  /// [`SplineOpts`]: struct.SplineOpts.html
  pub fn with_tension(x: f64, y: f64, tension: f64) -> Self {
    Point {
      x,
      y,
      tension: Some(tension),
    }
  }

  ///
  /// Tests the approximate equality of two points with default precision -
  /// [`DEFAULT_APPROX_EQ_PRECISION`]
  /// ```
  /// use cubic_spline::Point;
  ///
  /// assert!(Point::new(1.2,3.5).approx_eq(&[1.2, 3.5].into()));
  ///
  /// ```
  ///
  /// [`DEFAULT_APPROX_EQ_PRECISION`]: static.DEFAULT_APPROX_EQ_PRECISION.html
  pub fn approx_eq(&self, other: &Point) -> bool {
    ((self.x - other.x).abs() < DEFAULT_APPROX_EQ_PRECISION)
      && ((self.y - other.y).abs() < DEFAULT_APPROX_EQ_PRECISION)
  }

  ///
  /// Tests the approximate equality with specific precision
  ///
  /// ```
  /// use cubic_spline::Point;
  ///
  /// assert!(Point::new(1.0,1.0).approx_eq_with_precision(&[1.000_001, 1.0].into(), 0.000_1));
  /// assert!(Point::new(1.000_1,1.0).approx_eq_with_precision(&[1.0, 1.0].into(), 0.01));
  /// ```
  ///
  pub fn approx_eq_with_precision(&self, other: &Point, precision: f64) -> bool {
    ((self.x - other.x).abs() < precision) && ((self.y - other.y).abs() < precision)
  }

  ///
  /// Inverts the x-value of the point based on the width of the canvas.
  ///
  /// # Example
  /// ```
  /// use cubic_spline::Point;
  ///
  /// let mut p = Point::new(1.0, 3.0);
  /// p.invert_horizontally(3.0);
  ///
  /// assert_eq!(p.x, 2.0);
  /// assert_eq!(p.y, 3.0);
  ///
  /// ```
  pub fn invert_horizontally(&mut self, width: f64) {
    self.x = width - self.x;
  }

  ///
  /// Inverts the y-value of the point based on the height of the canvas.
  ///
  /// # Example
  /// ```
  /// use cubic_spline::Point;
  ///
  /// let mut p = Point::new(1.0, 3.0);
  /// p.invert_vertically(7.0);
  ///
  /// assert_eq!(p.x, 1.0);
  /// assert_eq!(p.y, 4.0);
  ///
  /// ```
  pub fn invert_vertically(&mut self, height: f64) {
    self.y = height - self.y;
  }
}

//
//
//
//
//////////////////////////////////////////////////////
// POINT FROM IMPL
//////////////////////////////////////////////////////

impl From<(f64, f64)> for Point {
  fn from(p: (f64, f64)) -> Self {
    Point::new(p.0, p.1)
  }
}

impl<'a> From<&'a Point> for Point {
  fn from(p: &'a Point) -> Self {
    p.clone()
  }
}

impl<'a, T: Copy> From<&'a T> for Point
where
  T: Into<Point>,
{
  fn from(p: &'a T) -> Self {
    (*p).into()
  }
}

impl<T> From<[T; 2]> for Point
where
  (T, T): Into<Point>,
{
  fn from([x, y]: [T; 2]) -> Self {
    (x, y).into()
  }
}

//
//
//
//
//////////////////////////////////////////////////////
// LIST OF POINTS OWN IMPL
//////////////////////////////////////////////////////
impl Points {
  ///
  /// Gets a reference to the underlying `Vec<Point>`.
  ///
  pub fn get_ref(&self) -> &Vec<Point> {
    &self.0
  }

  ///
  /// Gets a mutable reference to the underlying `Vec<Point>`.
  ///
  pub fn get_mut(&mut self) -> &mut Vec<Point> {
    &mut self.0
  }

  ///
  /// Consumes the `Points`, returning the wrapped `Vec<Point>`.
  ///
  pub fn into_inner(self) -> Vec<Point> {
    self.0
  }

  ///
  /// Similar to [`try_from`] but takes a flatten sequence of `f64` numbers
  /// where value at even index is `x` and value at odd index is `y`
  /// (e.g. `vec![12.0f64, 12.77, 15.3, 17.9]`, `[x,y,x,y,x...]`).
  ///
  /// [`try_from`]: trait.TryFrom.html#tymethod.try_from
  pub fn try_from_flatten<'a, I: IntoIterator<Item = &'a f64>>(into_f64_iter: I) -> Result<Self> {
    let mut v = Vec::new();

    let mut x = None;

    for point in into_f64_iter.into_iter() {
      if let Some(px) = x {
        v.push(Point::new(px, *point));
        x = None;
      } else {
        x = Some(*point);
      }
    }

    if x.is_some() {
      return Err(Error::MissingY);
    }
    if v.len() < 2 {
      return Err(Error::TooFewPoints);
    }

    Ok(Points(v))
  }

  ///
  /// Inverts the x-value of all points based on the width of the canvas.
  ///
  /// # Example
  /// ```
  /// use cubic_spline::Points;
  ///
  /// let mut pts = Points::from(&[(1.0, 3.0), (2.0, 2.0)]);
  /// pts.invert_horizontally(7.0);
  ///
  /// let inverted: Vec<(f64,f64)> = pts.into();
  /// assert_eq!(&[(6.0, 3.0), (5.0,2.0)].as_ref(), &inverted );
  ///
  /// ```
  pub fn invert_horizontally(&mut self, width: f64) {
    self.0.iter_mut().for_each(|p| p.invert_horizontally(width));
  }

  ///
  /// Inverts the y-value of all points based on the height of the canvas.
  ///
  /// # Example
  /// ```
  /// use cubic_spline::Points;
  ///
  /// let mut pts = Points::from(&[(1.0, 3.0), (2.0, 2.0)]);
  /// pts.invert_vertically(7.0);
  ///
  /// let inverted: Vec<(f64,f64)> = pts.into();
  /// assert_eq!(&[(1.0, 4.0), (2.0,5.0)].as_ref(), &inverted );
  ///
  /// ```
  pub fn invert_vertically(&mut self, height: f64) {
    self.0.iter_mut().for_each(|p| p.invert_vertically(height));
  }

  ///
  /// The main function that does all the work.
  ///
  /// Returns points of curve constructed within the range of passed points
  /// using cubic spline interpolation.
  ///
  /// # Example
  /// ```
  /// use cubic_spline::{Points, TryFrom, SplineOpts};
  ///
  /// let src_points = vec![(1.0, 1.0), (3.3, 2.7), (5.1, 0.9)];
  /// let prepared_points = Points::try_from(&src_points).expect("cant convert points");
  ///
  /// let options = SplineOpts::new()
  ///   .tension(0.5)
  ///   .num_of_segments(16);
  ///
  /// let calculated_points = prepared_points.calc_spline(&options).unwrap();
  ///
  /// assert_eq!(calculated_points.get_ref().len(), 33);
  /// ```
  pub fn calc_spline(&self, opts: &SplineOpts) -> Result<Points> {
    calc_spline(&self, opts)
  }
}

//
//
//
//
//////////////////////////////////////////////////////
// LIST OF POINTS FROM IMPL
//////////////////////////////////////////////////////

impl<I: IntoIterator> From<I> for Points
where
  I::Item: Into<Point>,
{
  fn from(points: I) -> Self {
    Points(points.into_iter().map(Into::into).collect())
  }
}

impl<I: IntoIterator> TryFrom<I> for Points
where
  I::Item: Into<Point>,
{
  type Error = Error;
  fn try_from(points: I) -> Result<Self> {
    let v: Vec<Point> = points.into_iter().map(Into::into).collect();
    if v.len() < 2 {
      return Err(Error::TooFewPoints);
    }
    Ok(Points(v))
  }
}

impl From<Points> for Vec<(f64, f64)> {
  fn from(pts: Points) -> Self {
    pts.get_ref().iter().map(|p| (p.x, p.y)).collect()
  }
}

impl From<Points> for Vec<[f64; 2]> {
  fn from(pts: Points) -> Self {
    pts.get_ref().iter().map(|p| [p.x, p.y]).collect()
  }
}

impl From<Points> for Vec<f64> {
  fn from(pts: Points) -> Self {
    let mut res = Vec::with_capacity(pts.0.len() * 2);
    pts.get_ref().iter().for_each(|p| {
      res.push(p.x);
      res.push(p.y);
    });
    res
  }
}

//
//
//
//
//////////////////////////////////////////////////////
// TESTS
//////////////////////////////////////////////////////
#[cfg(test)]
mod test {
  use crate::{Error, Points, TryFrom};

  fn points_eq(pp1: &Points, pp2: &Points) -> bool {
    pp1
      .get_ref()
      .iter()
      .zip(pp2.get_ref().iter())
      .all(|(p1, p2)| p1.approx_eq_with_precision(p2, 0.000_1))
  }

  #[test]
  fn from() {
    let src1 = vec![[1.2, 3.3], [122.2, 333.3]];
    let src2 = [[1.2, 3.3], [122.2, 333.3]];
    let src3 = [(1.2, 3.3), (122.2, 333.3)];
    let src4 = [1.2, 3.3, 122.2, 333.3];

    assert!(points_eq(&Points::from(&src1), &Points::from(&src2)));
    assert!(points_eq(&Points::from(&src1), &Points::from(&src3)));
    assert!(points_eq(
      &Points::from(&src1),
      &Points::try_from_flatten(&src4).unwrap(),
    ));

    let src5 = vec![[1.2, 3.3]];
    let src6 = [1.2, 3.3, 122.2];

    assert_eq!(Points::try_from(&src5).unwrap_err(), Error::TooFewPoints);
    assert_eq!(
      Points::try_from_flatten(&src6).unwrap_err(),
      Error::MissingY
    );
  }
}
