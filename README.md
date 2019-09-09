# cubic_spline

[![Crates.io](https://img.shields.io/crates/v/cubic_spline.svg)](https://crates.io/crates/cubic_spline/)
[![npm](https://img.shields.io/npm/v/cubic-spline-rs.svg?style=flat)](https://www.npmjs.com/package/cubic-spline-rs)

Interpolation methods for computation of cubic spline points within the range of a discrete set of known points.

[Online documentation](https://docs.rs/cubic_spline/0.7.8/cubic_spline/)
<br />
[Demo](https://emgyrz.github.io/cubic_spline/)


### Example
```rust
use cubic_spline::{CalcPoints, SplineOpts, SplineResult, SrcPoints};

let points = vec![(10.0, 200.0), (256.0, 390.0), (512.0, 10.0), (778.0, 200.0)];

let opts = SplineOpts {
  num_of_segments: 16,
  ..Default::default()
};

let pts: SrcPoints<(f64, f64)> = SrcPoints::new(&points);
let mut result = SplineResult::<(f64, f64)>::new();
pts.calc(&opts, &mut result);

assert_eq!(result.get().len(), 51);

//
// Same as:
//
use cubic_spline::{Spline};
let spline_points = Spline::from_tuples(&points, &opts);

assert_eq!(spline_points.len(), 51);
```



For now source and resulting points may be `Vec<f64> - (vec![x,y,x,y,...])` or `Vec<(f64, f64)> - (vec![(x,y),(x,y), ...])`.
For this types of points there are two helper functions `Spline::from_flatten_points` and `Spline::from_tuples`


### Custom points

If you allready have some points to avoid unnecessary copying, creating new `Vec` etc. you can implement `GetPoint` trait. And if you need some particular result implement `PushPoint`.
### Example
```rust
use cubic_spline::{CalcPoints,SrcPoints,SplineResult,PushPoint,GetPoint,SplineOpts};

struct MyPoint {
  pub top: f32,
  pub left: f32,
  pub label: Option<String>,
}

struct MyResult<T>(T);
struct MySrcPoint<T>(T);

impl<'a> GetPoint for MySrcPoint<SrcPoints<'a, MyPoint>> {
  fn get(&self, index: usize) -> Option<(f64, f64)> {
    self.0.pts().get(index).and_then(|p| {
      Some((f64::from(p.left), f64::from(p.top)))
    })
  }
  fn len(&self) -> usize {
    self.0.pts().len()
  }
}

impl PushPoint for MyResult<SplineResult<MyPoint>> {
  fn push_spline_point(&mut self, x: f64, y: f64) {
    let calculated_point = MyPoint { top: y as f32, left: x as f32, label: None };
    self.0.pts().push(calculated_point);
  }
}

impl<'a> CalcPoints for MySrcPoint<SrcPoints<'a, MyPoint>> {}


let points: Vec<MyPoint> = vec![];
let pts = MySrcPoint(SrcPoints::new(&points));
let mut result = MyResult(SplineResult::default());
pts.calc(&SplineOpts::default(), &mut result);

```


### Example for js
```js
import { getCurvePoints } from 'cubic-spline-rs'

const NUM_OF_SEGMENTS = 22

const points = [10.0, 200.0, 256.0, 390.0, 512.0, 10.0, 778.0, 200.0]

const curvePoints = getCurvePoints( points, {
  num_of_segments: NUM_OF_SEGMENTS, // *optional
  // tension: 0.5, // *optional
  // disallow_x_stepping_back: false, // *optional
} )

```


### Options
| Name                     |  Type  | Default | Description                                                           |
|--------------------------|:------:|:-------:|-----------------------------------------------------------------------|
| tension                  | `f64`  |  `0.5`  | Tension                                                               |
| num_of_segments          | `u32`  |  `16`   | Number of calculated points between known points                      |
| disallow_x_stepping_back | `bool` | `false` | If `true` checks that every x value of point is greater than previous |

```rust
use cubic_spline::{SplineOpts};
let opts = SplineOpts {
  tension: 0.6,
  ..Default.default()
}
```



##### Enjoy using!

### License

This module is [MIT licensed](./LICENSE).


