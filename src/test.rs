use super::{Spline, SplineOpts};

#[test]
fn compare_flatten_tst() {
  let points = vec![10.0, 200.0, 256.0, 390.0, 512.0, 10.0, 778.0, 200.0];

  #[allow(clippy::unreadable_literal)]
  let result = vec![
    10.0,
    200.0,
    18.60009765625,
    207.7001953125,
    28.91015625,
    218.5546875,
    40.75732421875,
    232.0068359375,
    53.96875,
    247.5,
    68.37158203125,
    264.4775390625,
    83.79296875,
    282.3828125,
    100.06005859375,
    300.6591796875,
    117.0,
    318.75,
    134.43994140625,
    336.0986328125,
    152.20703125,
    352.1484375,
    170.12841796875,
    366.3427734375,
    188.03125,
    378.125,
    205.74267578125,
    386.9384765625,
    223.08984375,
    392.2265625,
    239.89990234375,
    393.4326171875,
    256.0,
    390.0,
    256.0,
    390.0,
    271.70703125,
    380.86181640625,
    287.453125,
    365.87890625,
    303.23828125,
    345.88623046875,
    319.0625,
    321.71875,
    334.92578125,
    294.21142578125,
    350.828125,
    264.19921875,
    366.76953125,
    232.51708984375,
    382.75,
    200.0,
    398.76953125,
    167.48291015625,
    414.828125,
    135.80078125,
    430.92578125,
    105.78857421875,
    447.0625,
    78.28125,
    463.23828125,
    54.11376953125,
    479.453125,
    34.12109375,
    495.70703125,
    19.13818359375,
    512.0,
    10.0,
    512.0,
    10.0,
    528.83740234375,
    6.5673828125,
    546.58984375,
    7.7734375,
    565.05517578125,
    13.0615234375,
    584.03125,
    21.875,
    603.31591796875,
    33.6572265625,
    622.70703125,
    47.8515625,
    642.00244140625,
    63.9013671875,
    661.0,
    81.25,
    679.49755859375,
    99.3408203125,
    697.29296875,
    117.6171875,
    714.18408203125,
    135.5224609375,
    729.96875,
    152.5,
    744.44482421875,
    167.9931640625,
    757.41015625,
    181.4453125,
    768.66259765625,
    192.2998046875,
    778.0,
    200.0,
  ];
  let opts = SplineOpts {
    tension: 0.5,
    num_of_segments: 16,
    ..Default::default()
  };
  assert_eq!(Spline::from_flatten_points(&points, &opts), result);
}

#[test]
fn compare_tupoles_tst() {
  let points = vec![(10.0, 200.0), (256.0, 390.0), (512.0, 10.0), (778.0, 200.0)];

  #[allow(clippy::unreadable_literal)]
  let result: Vec<(f64, f64)> = vec![
    (10.0, 200.0),
    (18.60009765625, 207.7001953125),
    (28.91015625, 218.5546875),
    (40.75732421875, 232.0068359375),
    (53.96875, 247.5),
    (68.37158203125, 264.4775390625),
    (83.79296875, 282.3828125),
    (100.06005859375, 300.6591796875),
    (117.0, 318.75),
    (134.43994140625, 336.0986328125),
    (152.20703125, 352.1484375),
    (170.12841796875, 366.3427734375),
    (188.03125, 378.125),
    (205.74267578125, 386.9384765625),
    (223.08984375, 392.2265625),
    (239.89990234375, 393.4326171875),
    (256.0, 390.0),
    (256.0, 390.0),
    (271.70703125, 380.86181640625),
    (287.453125, 365.87890625),
    (303.23828125, 345.88623046875),
    (319.0625, 321.71875),
    (334.92578125, 294.21142578125),
    (350.828125, 264.19921875),
    (366.76953125, 232.51708984375),
    (382.75, 200.0),
    (398.76953125, 167.48291015625),
    (414.828125, 135.80078125),
    (430.92578125, 105.78857421875),
    (447.0625, 78.28125),
    (463.23828125, 54.11376953125),
    (479.453125, 34.12109375),
    (495.70703125, 19.13818359375),
    (512.0, 10.0),
    (512.0, 10.0),
    (528.83740234375, 6.5673828125),
    (546.58984375, 7.7734375),
    (565.05517578125, 13.0615234375),
    (584.03125, 21.875),
    (603.31591796875, 33.6572265625),
    (622.70703125, 47.8515625),
    (642.00244140625, 63.9013671875),
    (661.0, 81.25),
    (679.49755859375, 99.3408203125),
    (697.29296875, 117.6171875),
    (714.18408203125, 135.5224609375),
    (729.96875, 152.5),
    (744.44482421875, 167.9931640625),
    (757.41015625, 181.4453125),
    (768.66259765625, 192.2998046875),
    (778.0, 200.0),
  ];

  let opts = SplineOpts {
    tension: 0.5,
    num_of_segments: 16,
    ..Default::default()
  };

  let spline_points = Spline::from_tuples(&points, &opts);
  // println!("{:?}", spline_points);

  assert_eq!(spline_points, result);
}