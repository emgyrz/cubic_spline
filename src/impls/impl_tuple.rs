use crate::{CalcPoints, GetPoint, PushPoint, SplineResult, SrcPoints};

impl<'a> GetPoint for SrcPoints<'a, (f64, f64)> {
  fn get(&self, index: usize) -> Option<(f64, f64)> {
    self.pts().get(index).cloned()
  }
  fn len(&self) -> usize {
    self.pts().len()
  }
}

impl PushPoint for SplineResult<(f64, f64)> {
  fn push_spline_point(&mut self, x: f64, y: f64) {
    self.pts().push((x, y));
  }
}

impl<'a> CalcPoints for SrcPoints<'a, (f64, f64)> {}
