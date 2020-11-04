use crate::{Points, SplineOpts};
use wasm_bindgen::prelude::*;

#[allow(clippy::too_many_arguments, non_snake_case)]
#[wasm_bindgen]
pub fn getCurvePoints(
  pts: Vec<f64>,
  num_of_segments: Option<u32>,
  tension: Option<f64>,
  custom_tensions: Option<Vec<f64>>,
  invert_x_with_width: Option<f64>,
  invert_y_with_height: Option<f64>,
  hidden_point_at_start: Option<Vec<f64>>,
  hidden_point_at_end: Option<Vec<f64>>,
) -> Vec<f64> {
  let mut b = SplineOpts::new();

  if let Some(t) = tension {
    b = b.tension(t);
  }

  if let Some(n) = num_of_segments {
    b = b.num_of_segments(n);
  }

  if let Some(s) = hidden_point_at_start {
    if s.len() >= 2 {
      b = b.hidden_point_at_start((s[0], s[1]));
    }
  }

  if let Some(s) = hidden_point_at_end {
    if s.len() >= 2 {
      b = b.hidden_point_at_end((s[0], s[1]));
    }
  }

  let mut pts = Points::try_from_flatten(&pts).unwrap();

  if let Some(w) = invert_x_with_width {
    pts.invert_horizontally(w);
  }

  if let Some(h) = invert_y_with_height {
    pts.invert_vertically(h);
  }

  if let Some(ct) = custom_tensions {
    ct.iter().enumerate().for_each(|(i, t)| {
      if *t > -100.0 {
        pts.get_mut().get_mut(i).iter_mut().for_each(|p| {
          p.tension = Some(*t);
        });
      }
    });
  }

  pts.calc_spline(&b).unwrap().into()
}
