use super::{result, GetPoint, SplineOpts};

/// Trait that includes one provided method to calculate curve points
/// ```ignore
/// impl<'a> CalcPoints for SrcPoints<'a, (f64, f64)> {}
///
/// SrcPoints::new(&points).calc(&spline_options, &mut result_vec);
///
/// ```
pub trait CalcPoints {
  fn calc<R: result::PushPoint>(&self, opts: &SplineOpts, result: &mut R)
  where
    Self: GetPoint,
  {
    let SplineOpts {
      tension,
      num_of_segments,
      disallow_x_stepping_back,
    } = *opts;

    let num_of_segments_f64 = f64::from(num_of_segments);

    for i in 0..(self.len()) {
      let (prev, current, next, next2) = if let Some(p) = self.points_to_calc(i) {
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
        let y = c1 * current.1 + c2 * next.1 + c3 * t1y + c4 * t2y;

        if disallow_x_stepping_back {
          if let Some(last) = self.last() {
            let last_x = last.0;
            if x < last_x {
              x = last_x;
            }
          }
        }
        result.push_spline_point(x, y);
      }
    }
  }
}
