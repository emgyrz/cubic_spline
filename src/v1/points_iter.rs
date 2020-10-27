use crate::{Point, Points, SplineOpts};

pub(crate) type PointsToCalc<'a> = (&'a Point, &'a Point, &'a Point, &'a Point);

pub(crate) struct PointsIter<'a> {
  max_index: usize,
  index: usize,
  points: &'a Points,
  current: [&'a Point; 4],
  hidden_point_at_end: Option<&'a Point>,
}

impl<'a> PointsIter<'a> {
  pub(crate) fn new(points: &'a Points, opts: &'a SplineOpts) -> Self {
    let pts = points.get_ref();
    let curr = &pts[0];
    let prev = opts.hidden_point_at_start.as_ref().unwrap_or(&curr);
    let next = &pts[1];
    let next2 = pts.get(2).unwrap_or(&next);

    let current = [prev, curr, next, next2];

    PointsIter {
      max_index: pts.len() - 1,
      index: 0,
      points,
      current,
      hidden_point_at_end: opts.hidden_point_at_end.as_ref(),
    }
  }

  fn get_points_to_calc(&self) -> PointsToCalc<'a> {
    let c = self.current;
    (&c[0], &c[1], &c[2], &c[3])
  }

  fn update_current(&mut self) {
    self.current[0] = self.current[1];
    self.current[1] = self.current[2];
    self.current[2] = self.current[3];

    let pts = self.points.get_ref();

    let last = if self.index == self.max_index - 1 {
      self.hidden_point_at_end.unwrap_or(&self.current[2])
    } else {
      &pts[self.index + 2]
    };

    self.current[3] = last;
  }
}

impl<'a> Iterator for PointsIter<'a> {
  type Item = PointsToCalc<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index == self.max_index {
      return None;
    }

    let points_to_calc = self.get_points_to_calc();
    self.index += 1;

    // Всё из-за того, что при вызове `self.update_current()` расчитывается следующее состояние
    // Следующее, потому что в самом начале расчитывается первое состояние,
    // чтобы не проверять каждый раз на нулевой индекс и не хранить `hidden_point_at_start`
    if self.index != self.max_index {
      self.update_current();
    }

    Some(points_to_calc)
  }
}
