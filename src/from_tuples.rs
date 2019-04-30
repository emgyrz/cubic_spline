use super::SplineOpts;

pub fn get_curve_points(points: &[(f64, f64)], opts: &SplineOpts) -> Vec<(f64, f64)> {
  let SplineOpts {
    tension,
    num_of_segments,
    disallow_x_stepping_back,
  } = *opts;

  let mut pts: Vec<(f64, f64)> = points.into();

  let length = pts.len();
  if length == 1 {
    return pts;
  }

  pts.insert(0, pts[0]);
  pts.push(*pts.last().unwrap());

  let mut result: Vec<(f64, f64)> = Vec::with_capacity(pts.len() * num_of_segments as usize);
  let num_of_segments_f64 = f64::from(num_of_segments);

  for i in 1..(pts.len() - 2) {
    for t in 0..=num_of_segments {
      let next = pts[i + 1];
      let next2 = pts[i + 2];
      let prev = pts[i - 1];
      let current = pts[i];

      let t1x = (next.0 - prev.0) * tension;
      let t2x = (next2.0 - current.0) * tension;
      // [(x,y),(x,y),(x,y),]
      let t1y = (next.1 - prev.1) * tension;
      let t2y = (next2.1 - current.1) * tension;

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
        if let Some(last) = result.last() {
          let last_x = last.0;
          if x < last_x {
            x = last_x;
          }
        }
      }

      result.push((x, y));
    }
  }

  result
}
