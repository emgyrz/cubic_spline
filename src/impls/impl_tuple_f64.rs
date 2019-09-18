use crate::{CalcPoints, GetPoint, PushPoint, SplineResult, SrcPoints};

// use crate::points::PointsToCalc;

impl<'a> GetPoint for SrcPoints<'a, (f64, f64)> {
  fn get(&self, index: usize) -> Option<(f64, f64)> {
    self.pts().get(index).cloned()
  }
  fn len(&self) -> usize {
    self.pts().len()
  }

  // fn points_to_calc(&self, index: usize) -> Option<PointsToCalc<f64>> {
  //   let indpp = index + 2;
  //   let len = self.pts().len();
  //   if indpp > len {
  //     return None;
  //   }

  //   let current = unsafe { self.get_unchecked(index) };
  //   let next = unsafe { self.get_unchecked(index + 1) };

  //   let prev = if index == 0 {
  //     current
  //   } else {
  //     unsafe { self.get_unchecked(index - 1) }
  //   };

  //   let next2 = if indpp == len {
  //     next
  //   } else {
  //     unsafe { self.get_unchecked(index + 2) }
  //   };
  //   Some((prev, current, next, next2))
  // }
}

impl PushPoint for SplineResult<(f64, f64)> {
  fn push_spline_point(&mut self, x: f64, y: f64) {
    self.pts().push((x, y));
  }
}

impl<'a> SrcPoints<'a, (f64, f64)> {
  pub unsafe fn get_unchecked(&self, index: usize) -> (f64, f64) {
    *self.pts().get_unchecked(index)
  }
}

impl<'a> CalcPoints for SrcPoints<'a, (f64, f64)> {}
