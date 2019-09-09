use super::{CalcPoints, Spline, SplineOpts, SplineResult, SrcPoints};

mod data_flatten;
mod data_tuples;


#[test]
fn bench1() {
  use std::time::Instant;
  let points = vec![10.0, 200.0, 256.0, 390.0, 512.0, 10.0, 778.0, 200.0];
  // let points = vec![(10.0, 200.0), (256.0, 390.0), (512.0, 10.0), (778.0, 200.0)];

  let opts = SplineOpts {
    tension: 0.5,
    num_of_segments: 16,
    ..Default::default()
  };

  let mut v = Vec::new();

  let start = Instant::now();

  let points2 = vec![(10.0, 200.0), (256.0, 390.0), (512.0, 10.0), (778.0, 200.0)];

  for _ in 0..10000 {
    let pts = SrcPoints::<(f64, f64)>::new(&points2);
    let mut result = SplineResult::<(f64, f64)>::with_capacity(points.len() * 16);
    pts.calc(&opts, &mut result);
    v.push(result)
    // v.push(Spline::from_flatten_points(&points, &opts))
  }

  let duration = start.elapsed();

  println!(">>>>>>>>>>>>   ms {:?}", duration.as_millis());
  println!("{:?}", v.len());
}

#[test]
fn compare_flatten_tst() {

  let result_points = data_flatten::result();
  let points = data_flatten::points();

  let opts = SplineOpts {
    tension: 0.5,
    num_of_segments: 16,
    ..Default::default()
  };
  assert_eq!(
    Spline::from_flatten_points(&points, &opts),
    result_points
  );

  let mut result = SplineResult::<f64>::default();
  SrcPoints::<f64>::new(&points).calc(&opts, &mut result);
  assert_eq!(result.get(), result_points);
}

#[test]
fn compare_tupoles_tst() {
  let points = data_tuples::points();
  let result_points = data_tuples::result();

  let opts = SplineOpts {
    tension: 0.5,
    num_of_segments: 16,
    ..Default::default()
  };

  let spline_points = Spline::from_tuples(&points, &opts);

  assert_eq!(spline_points, result_points);

  let pts = SrcPoints::<(f64, f64)>::new(&points);
  let mut result = SplineResult::<(f64, f64)>::default();
  pts.calc(&opts, &mut result);
  assert_eq!(spline_points, result.get());
}
