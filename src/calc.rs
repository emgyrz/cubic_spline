use super::{result, GetPoint, SplineOpts};

///
/// Trait that includes one provided method to calculate curve points
/// ```ignore
/// impl<'a> CalcPoints for SrcPoints<'a, (f64, f64)> {}
///
/// SrcPoints::new(&points).calc(&spline_options, &mut result_vec);
///
/// ```
#[deprecated(since = "1.0.0")]
pub trait CalcPoints {
  fn calc<R: result::PushPoint>(&self, opts: &SplineOpts, result: &mut R)
  where
    Self: GetPoint,
  {
    let SplineOpts {
      tension,
      num_of_segments,
      invert_y_with_height,
      invert_x_with_width,
      ..
      // hidden_point_at_start,
      // hidden_point_at_end,
    } = *opts;

    let hidden_point_at_start = opts.hidden_point_at_start.as_ref().map(|p| (p.x, p.y));
    let hidden_point_at_end = opts.hidden_point_at_end.as_ref().map(|p| (p.x, p.y));

    let need_invert_y = invert_y_with_height.is_some();
    let canvas_height = invert_y_with_height.unwrap_or_default() as f64;
    let need_invert_x = invert_x_with_width.is_some();
    let canvas_width = invert_x_with_width.unwrap_or_default() as f64;

    let num_of_segments_f64 = f64::from(num_of_segments);

    for i in 0..(self.len()) {
      let (prev, current, next, next2) =
        if let Some(p) = self.points_to_calc(i, &hidden_point_at_start, &hidden_point_at_end) {
          p
        } else {
          continue;
        };

      let t1x = (next.0 - prev.0) * tension;
      let t2x = (next2.0 - current.0) * tension;
      let t1y = (next.1 - prev.1) * tension;
      let t2y = (next2.1 - current.1) * tension;

      for t in 0..=num_of_segments {
        let st = f64::from(t) / num_of_segments_f64;
        let st_pow2 = st.powi(2);
        let st_pow3 = st.powi(3);
        let st_pow2x3 = 3.0 * st_pow2;
        let st_pow3x2 = 2.0 * st_pow3;

        let c1 = st_pow3x2 - st_pow2x3 + 1.0;
        let c2 = -st_pow3x2 + st_pow2x3;
        let c3 = st_pow3 - 2.0 * st_pow2 + st;
        let c4 = st_pow3 - st_pow2;

        let mut x = c1 * current.0 + c2 * next.0 + c3 * t1x + c4 * t2x;
        let mut y = c1 * current.1 + c2 * next.1 + c3 * t1y + c4 * t2y;

        if need_invert_x {
          x = canvas_width - x;
        }

        if need_invert_y {
          y = canvas_height - y;
        }

        result.push_spline_point(x, y);
      }
    }
  }
}
