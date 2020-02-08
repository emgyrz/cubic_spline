# cubic_spline

[![Crates.io](https://img.shields.io/crates/v/cubic_spline.svg)](https://crates.io/crates/cubic_spline/)
[![npm](https://img.shields.io/npm/v/cubic-spline-rs.svg)](https://www.npmjs.com/package/cubic-spline-rs)

Interpolation methods for computation of cubic spline points within the range of a discrete set of known points.

[Online documentation](https://docs.rs/cubic_spline/0.9.1/cubic_spline/)
<br />
[Demo](https://emgyrz.github.io/cubic_spline/)


#### Example
```rust
use cubic_spline::{CalcPoints, SplineOptsBuilder, SplineResult, SrcPoints};

let points = vec![(10.0, 200.0), (256.0, 390.0), (512.0, 10.0), (778.0, 200.0)];

let opts = SplineOptsBuilder::new()
  .num_of_segments(16)
  .take();

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
#### Example
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

See [here](https://github.com/emgyrz/cubic_spline/tree/master/src/impls) for implementation example


### Example for js
```js
import { getCurvePoints } from 'cubic-spline-rs'

const NUM_OF_SEGMENTS = 22

const points = [10.0, 200.0, 256.0, 390.0, 512.0, 10.0, 778.0, 200.0]

const curvePoints = getCurvePoints( points, {
  num_of_segments: NUM_OF_SEGMENTS, // *optional
  // tension: 0.5, // *optional
} )

```

If you want to draw result points to canvas - code like this:
```js
const ctx = getMyCanvas2DContext()

ctx.beginPath()
ctx.lineWidth = 3
ctx.strokeStyle = '#ffcc00'

ctx.moveTo(curvePoints[0], curvePoints[1])
const length = curvePoints.length - 1
for (let i = 2; i < length; i += 2) {
  ctx.lineTo(curvePoints[i], curvePoints[i + 1])
}

ctx.stroke()
ctx.closePath()
```
See example [here](./www/src/Spline.ts).




### Options
| Name                 | Type  | Default | Description                                      |
| -------------------- | :---: | :-----: | ------------------------------------------------ |
| tension              | `f64` | `0.5`   | Tension                                          |
| num_of_segments      | `u32` | `16`    | Number of calculated points between known points |
| invert_x_with_width  | `Option<u32>` | `None`   | If set to `Some(canvas_width)` generated points will be inverted by X-axis.                                          |
| invert_y_with_height | `Option<u32>` | `None`   | If set to `Some(canvas_height)` generated points will be inverted by Y-axis.                                          |


```rust
use cubic_spline::{SplineOpts, SplineOptsBuilder};
let opts = SplineOpts {
  tension: 0.6,
  ..Default.default()
};

// or use builder

let opts2 = SplineOptsBuilder::new()
  .num_of_segments(54)
  .invert_y_with_height(1080)
  .take();

```



##### Enjoy using!

### License

This module is [MIT licensed](./LICENSE).


