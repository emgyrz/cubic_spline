use super::SplineOpts;

pub struct Points<'a, T> {
  pts: &'a [T],
  pub result: Vec<T>,
}

pub type PointsToCalc<T> = ((T, T), (T, T), (T, T), (T, T));

impl<'a> Points<'a, f64> {
  pub fn new(pts: &'a [f64], len: Option<usize>) -> Self {
    Points {
      pts,
      result: Vec::with_capacity(len.unwrap_or(pts.len())),
    }
  }
}
impl<'a> Points<'a, (f64, f64)> {
  pub fn new(pts: &'a [(f64, f64)], len: Option<usize>) -> Self {
    Points {
      pts,
      result: Vec::with_capacity(len.unwrap_or(pts.len())),
    }
  }
}

pub trait CalcPoints {
  fn calc(&mut self, opts: &SplineOpts)
  where
    Self: GetPoint,
  {
    let SplineOpts {
      tension,
      num_of_segments,
      disallow_x_stepping_back,
    } = *opts;

    let num_of_segments_f64 = f64::from(num_of_segments);

    for i in 0..(self.len() - 1) {
      for t in 0..=num_of_segments {
        // let (prev, current, next, next2) = if let Some(p) = self.points_to_calc(i) {
        //   p
        // } else {
        //   continue;
        // };
        let (prev, current, next, next2) = self.points_to_calc(i).unwrap();

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
          if let Some(last) = self.last() {
            let last_x = last.0;
            if x < last_x {
              x = last_x;
            }
          }
        }
        self.push_to_result(x, y);
      }
    }
  }
}

// TODO: Copy????
pub trait GetPoint {
  fn len(&self) -> usize;
  fn get(&self, index: usize) -> Option<(f64, f64)>;
  fn last(&self) -> Option<(f64, f64)> {
    self.get(self.len() - 1)
  }
  fn push_to_result(&mut self, x: f64, y: f64);
  fn points_to_calc(&self, index: usize) -> Option<PointsToCalc<f64>> {
    let current = if let Some(curr) = self.get(index) {
      curr
    } else {
      return None;
    };

    let next = if let Some(n) = self.get(index + 1) {
      n
    } else {
      return None;
    };

    let prev = if index == 0 {
      current
    } else {
      self.get(index - 1).unwrap_or_else(|| current)
    };

    let next2 = self.get(index + 2).unwrap_or_else(|| next);

    // TODO: check speed with refs
    Some((prev, current, next, next2))
  }
}

impl<'a> GetPoint for Points<'a, (f64, f64)> {
  fn get(&self, index: usize) -> Option<(f64, f64)> {
    self.pts.get(index).cloned()
  }
  fn len(&self) -> usize {
    self.pts.len()
  }
  fn push_to_result(&mut self, x: f64, y: f64) {
    self.result.push((x, y));
  }
}

impl<'a> GetPoint for Points<'a, f64> {
  fn get(&self, index: usize) -> Option<(f64, f64)> {
    let x_ind = index * 2;
    let x = self.pts.get(x_ind);
    let y = self.pts.get(x_ind + 1);
    x.and(y).map(|y| (*x.unwrap(), *y))
  }
  fn len(&self) -> usize {
    self.pts.len() / 2
  }
  fn push_to_result(&mut self, x: f64, y: f64) {
    self.result.push(x);
    self.result.push(y);
  }
}

impl<'a> CalcPoints for Points<'a, f64> {}
impl<'a> CalcPoints for Points<'a, (f64, f64)> {}
