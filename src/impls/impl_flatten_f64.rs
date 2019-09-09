use crate::{CalcPoints, GetPoint, PushPoint, SplineResult, SrcPoints};

impl<'a> GetPoint for SrcPoints<'a, f64> {
  fn get(&self, index: usize) -> Option<(f64, f64)> {
    let x_ind = index * 2;
    let x = self.pts().get(x_ind);
    let y = self.pts().get(x_ind + 1);
    x.and(y).map(|y| (*x.unwrap(), *y))
  }
  fn len(&self) -> usize {
    self.pts().len() / 2
  }
}

impl PushPoint for SplineResult<f64> {
  fn push_spline_point(&mut self, x: f64, y: f64) {
    self.pts().push(x);
    self.pts().push(y);
  }
}

impl<'a> CalcPoints for SrcPoints<'a, f64> {}
