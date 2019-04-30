use super::SplineOpts;

pub fn get_curve_points(points: &[f64], opts: &SplineOpts) -> Vec<f64> {
  let SplineOpts {
    tension,
    num_of_segments,
    disallow_x_stepping_back,
  } = *opts;

  let mut pts: Vec<f64> = points.into();
  let length = pts.len();
  if length == 0 || length == 1 {
    return pts;
  }
  if pts.len() % 2 != 0 {
    pts.truncate(pts.len() - 1);
  }
  if pts.len() == 2 {
    return pts;
  }

  pts.insert(0, pts[1]);
  pts.insert(0, pts[1]);
  pts.push(pts[pts.len() - 2]);
  pts.push(pts[pts.len() - 2]);

  let mut result = Vec::with_capacity(pts.len() * num_of_segments as usize);
  let num_of_segments_f64 = f64::from(num_of_segments);
  let mut i = 2;
  while i < (pts.len() - 4) {
    for t in 0..=num_of_segments {
      let t1x = (pts[i + 2] - pts[i - 2]) * tension;
      let t2x = (pts[i + 4] - pts[i]) * tension;

      let t1y = (pts[i + 3] - pts[i - 1]) * tension;
      let t2y = (pts[i + 5] - pts[i + 1]) * tension;

      let st = f64::from(t) / num_of_segments_f64;

      let st_pow2 = st.powi(2);
      let st_pow3 = st.powi(3);
      let st_pow2x3 = 3.0 * st_pow2;
      let st_pow3x2 = 2.0 * st_pow3;

      let c1 = st_pow3x2 - st_pow2x3 + 1.0;
      let c2 = -st_pow3x2 + st_pow2x3;
      let c3 = st_pow3 - 2.0 * st_pow2 + st;
      let c4 = st_pow3 - st_pow2;

      let mut x = c1 * pts[i] + c2 * pts[i + 2] + c3 * t1x + c4 * t2x;
      let y = c1 * pts[i + 1] + c2 * pts[i + 3] + c3 * t1y + c4 * t2y;

      if disallow_x_stepping_back && result.len() >= 2 {
        let last_x = result[result.len() - 2];
        if x < last_x {
          x = last_x;
        }
      }

      result.push(x);
      result.push(y);
    }
    i += 2;
  }

  result
}
