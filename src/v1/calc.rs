use crate::{v1::points_iter::PointsIter, Error, Point, Points, Result, SplineOpts};

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
/// let calculated_points = prepared_points
///   .calc_spline(&options)
///   .expect("cant construct spline points");
///
/// assert_eq!(calculated_points.get_ref().len(), 33);
/// ```
pub(crate) fn calc_spline(points: &Points, opts: &SplineOpts) -> Result<Points> {
  let points_len = points.get_ref().len();

  if points_len < 2 {
    return Err(Error::TooFewPoints);
  }

  let SplineOpts {
    tension: tension_from_opt,
    num_of_segments,
    ..
  } = *opts;

  let num_of_segments_f64 = f64::from(num_of_segments);

  // количество сегментов на промежутках между точками
  // умноженное на количество промежутков
  // плюс последняя завершающая точка, т.к. функция расчитывает от точки и до точки не включительно
  let generated_count = (points_len - 1) * (num_of_segments as usize) + 1;

  let mut result: Vec<Point> = Vec::with_capacity(generated_count);

  let iter = PointsIter::new(points, opts);

  for (prev, curr, next, next2) in iter {
    let tension = curr.tension.unwrap_or(tension_from_opt);

    let t1x = (next.x - prev.x) * tension;
    let t2x = (next2.x - curr.x) * tension;
    let t1y = (next.y - prev.y) * tension;
    let t2y = (next2.y - curr.y) * tension;

    for t in 0..num_of_segments {
      let st = f64::from(t) / num_of_segments_f64;
      let st_pow2 = st.powi(2);
      let st_pow3 = st.powi(3);
      let st_pow2x3 = 3.0 * st_pow2;
      let st_pow3x2 = 2.0 * st_pow3;

      let c1 = st_pow3x2 - st_pow2x3 + 1.0;
      let c2 = -st_pow3x2 + st_pow2x3;
      let c3 = st_pow3 - 2.0 * st_pow2 + st;
      let c4 = st_pow3 - st_pow2;

      let x = c1 * curr.x + c2 * next.x + c3 * t1x + c4 * t2x;
      let y = c1 * curr.y + c2 * next.y + c3 * t1y + c4 * t2y;

      result.push(Point::new(x, y));
    }
  }

  // проверка лишняя. чтобы не писать unwrap
  if let Some(last) = points.get_ref().last() {
    // нужно добавить последнюю, потому что функция расчитывает точки
    // в промежутке между point1 и point2 включая первую, но не включая крайнюю с конца
    result.push(Point::new(last.x, last.y));
  }

  Ok(Points::from(result))
}
