pub struct SrcPoints<'a, T> {
  pts: &'a [T],
}

pub(crate) type PointsToCalc<T> = ((T, T), (T, T), (T, T), (T, T));

impl<'a, T> SrcPoints<'a, T> {
  pub fn new(pts: &'a [T]) -> Self {
    SrcPoints { pts }
  }
  pub fn pts(&self) -> &'a [T] {
    self.pts
  }
}

#[allow(clippy::len_without_is_empty)]
pub trait GetPoint {
  fn len(&self) -> usize;
  fn get(&self, index: usize) -> Option<(f64, f64)>;
  fn last(&self) -> Option<(f64, f64)> {
    self.get(self.len() - 1)
  }
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

    Some((prev, current, next, next2))
  }
}
