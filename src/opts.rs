pub struct SplineOpts {
  pub disallow_x_stepping_back: bool,
  pub tension: f64,
  pub num_of_segments: u32,
}

impl Default for SplineOpts {
  fn default() -> Self {
    SplineOpts {
      disallow_x_stepping_back: false,
      tension: 0.5,
      num_of_segments: 16,
    }
  }
}
