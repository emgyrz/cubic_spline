use crate::{CalcPoints, Error, Points, SplineOpts, SplineResult, SrcPoints, TryFrom};

mod data_flatten;
mod data_tuples;

#[test]
fn bench1() {
  use std::time::Instant;

  let points = vec![10.0, 200.0, 256.0, 390.0, 512.0, 10.0, 778.0, 200.0];

  let opts = SplineOpts::default();

  let mut v = Vec::new();

  let start = Instant::now();

  let points2 = vec![(10.0, 200.0), (256.0, 390.0), (512.0, 10.0), (778.0, 200.0)];

  for _ in 0..10000 {
    let pts = SrcPoints::<(f64, f64)>::new(&points2);

    let mut result = SplineResult::<(f64, f64)>::with_capacity(points.len() * 16);

    pts.calc(&opts, &mut result);

    v.push(result)
  }

  let duration = start.elapsed();

  println!(">>>>>>>>>>>>   ms {:?}", duration.as_millis());

  // println!("{:?}", v.len());
}

// TODO: del
// #[test]
// fn compare_flatten_tst_old() {
//   let result_points = data_flatten::result();
//
//   let points = data_flatten::points();
//
//   let opts = SplineOpts::default();
//
//   assert_eq!(Spline::from_flatten_points(&points, &opts), result_points);
//
//   let mut result = SplineResult::<f64>::default();
//
//   SrcPoints::<f64>::new(&points).calc(&opts, &mut result);
//
//   assert_eq!(result.get(), result_points);
// }

// TODO: del
// #[test]
// fn compare_tuples_tst_old() {
//   let points = data_tuples::points();
//   let result_points = data_tuples::result();
//   let opts = SplineOpts::default();
//   let spline_points = Spline::from_tuples(&points, &opts);
//
//   assert_eq!(spline_points, result_points);
//
//   let pts = SrcPoints::<(f64, f64)>::new(&points);
//   let mut result = SplineResult::<(f64, f64)>::default();
//   pts.calc(&opts, &mut result);
//
//   assert_eq!(spline_points, result.get());
// }

#[test]
fn calc_spline_tst() {
  let points = data_tuples::points();
  let result_points = data_tuples::result();
  let opts = SplineOpts::default();

  let spline_points = Points::from(&points).calc_spline(&opts).unwrap();

  assert_eq!(result_points, Into::<Vec<(f64, f64)>>::into(spline_points));

  let pts_from_flatten = Points::try_from_flatten(&data_flatten::points())
    .unwrap()
    .calc_spline(&opts)
    .unwrap();

  assert_eq!(
    &Into::<Vec<f64>>::into(pts_from_flatten),
    &data_flatten::result()
  );
}

#[test]
fn calc_spline_err() {
  let opts = SplineOpts::new();
  let zero: Vec<(f64, f64)> = vec![];

  assert_eq!(Points::try_from(&zero).unwrap_err(), Error::TooFewPoints,);

  let zero_res = Points::from(&zero).calc_spline(&opts);
  assert_eq!(zero_res.unwrap_err(), Error::TooFewPoints);

  let one = vec![[1.0, 22_123.123]];
  assert_eq!(Points::try_from(&one).unwrap_err(), Error::TooFewPoints);
  assert_eq!(Points::try_from(&zero).unwrap_err(), Error::TooFewPoints);

  let one_res = Points::from(&one).calc_spline(&opts);
  assert_eq!(one_res.unwrap_err(), Error::TooFewPoints);
}

#[test]
fn calc_spline_flatten_err() {
  let opts = SplineOpts::new();

  assert_eq!(
    Points::try_from_flatten(&vec![]).unwrap_err(),
    Error::TooFewPoints,
  );

  assert_eq!(
    Points::try_from_flatten(&vec![1.1]).unwrap_err(),
    Error::MissingY,
  );

  assert_eq!(
    Points::try_from_flatten(&vec![1.1, 2.2]).unwrap_err(),
    Error::TooFewPoints,
  );

  assert_eq!(
    Points::try_from_flatten(&vec![1.1, 2.2, 3.3]).unwrap_err(),
    Error::MissingY,
  );

  let two_points = Points::try_from_flatten(&vec![1.1, 2.2, 3.3, 4.4]).unwrap();
  let two_points_res = two_points.calc_spline(&opts).unwrap();
  assert_eq!(two_points_res.get_ref().len(), 17,);
}

#[test]
fn calc_spline_arr_tst() {
  let points = data_tuples::points_arr();
  let result_points = data_tuples::result_arr();

  let opts = Default::default();

  let mut pts = Points::from(&points);
  pts.invert_vertically(400.0);
  let spline_points = pts.calc_spline(&opts).unwrap();

  assert_eq!(result_points, Into::<Vec<f64>>::into(spline_points));
  pts.invert_vertically(400.0);
  // pts.invert_horizontally(900.0);

  assert_eq!(
    pts.calc_spline(&opts).unwrap().get_ref().len() * 2,
    result_points.len()
  );

  let pts_from_flatten = Points::try_from_flatten(&data_flatten::points())
    .unwrap()
    .calc_spline(&SplineOpts::default())
    .unwrap();

  assert_eq!(
    &Into::<Vec<f64>>::into(pts_from_flatten),
    &data_flatten::result()
  );
}

#[test]
fn invert_tst() {
  let points = data_tuples::points_arr();
  let mut pts = Points::from(&points);
  let res1 = pts.calc_spline(&SplineOpts::default()).unwrap();

  pts.invert_vertically(1000.0);
  pts.invert_horizontally(1000.0);

  let res2 = pts.calc_spline(&SplineOpts::default()).unwrap();

  assert_eq!(res1.get_ref().len(), res2.get_ref().len())
}
