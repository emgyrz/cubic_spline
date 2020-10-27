# cubic_spline

[![Crates.io](https://img.shields.io/crates/v/cubic_spline.svg)](https://crates.io/crates/cubic_spline/)
[![npm](https://img.shields.io/npm/v/cubic-spline-rs.svg)](https://www.npmjs.com/package/cubic-spline-rs)

Interpolation method for computation of cubic spline points within
the range of a discrete set of known points.

[Online documentation](https://docs.rs/cubic_spline/0.9.8/cubic_spline/)
<br />
<br />
[Demo](https://emgyrz.github.io/cubic_spline/)


### Example
```rust
use cubic_spline::{Points, Point, SplineOpts, TryFrom};

let source = vec![(10.0, 200.0), (256.0, 390.0), (512.0, 10.0), (778.0, 200.0)];

let opts = SplineOpts::new().tension(0.5);

let mut points = Points::try_from(&source).expect("expect valid points but");
let result = points.calc_spline(&opts).expect("cant construct spline points");

assert_eq!(result.get_ref().len(), 49);

points.get_mut().push(Point::new(7.7, 1.3));
points.get_mut()[1].x += 0.79;
points.invert_vertically(400.0);

assert_eq!(points.get_ref()[1].y, 10.0);

let calculated_points = points
  .calc_spline(&opts.num_of_segments(33))
  .unwrap();

assert_eq!(calculated_points.into_inner().len(), 133);

```

For information on how a curve can be constructed and which points to accept,
see the appropriate structures.



## Custom points

If you already have some points you can implement `From` trait for `Point`
struct and pass your points directly.

### Example
```rust
use cubic_spline::{SplineOpts, Point, Points};

#[derive(Default)]
struct MyPoint {
  vertical: u8,
  horizontal: u8,
  color: String,
}

impl<'a> From<&'a MyPoint> for Point {
  fn from(p: &'a MyPoint) -> Self {
    Point::new(p.horizontal as f64, p.vertical as f64)
  }
}

let my_points: Vec<MyPoint> = vec![MyPoint::default(),MyPoint::default()];
let spline = Points::from(&my_points)
  .calc_spline(&SplineOpts::default())
  .unwrap();

assert_eq!(spline.get_ref().len(), 17);

```


### Example for JS
```js
import { getCurvePoints } from 'cubic-spline-rs'

const NUM_OF_SEGMENTS = 22

const points = [10.0, 200.0, 256.0, 390.0, 512.0, 10.0, 778.0, 200.0]

const curvePoints = getCurvePoints( points, {
  num_of_segments: NUM_OF_SEGMENTS, // *optional
  // tension: 0.5, // *optional
  // ...  
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
| Name                  | Type                | Default | Description                                                                                 |
| --------------------- | :-----------------: | :-----: | ------------------------------------------------------------------------------------------- |
| tension               | `f64`               | `0.5`   | Tension                                                                                     |
| num_of_segments       | `u32`               | `16`    | Number of calculated points between known points                                            |
| hidden_point_at_start | `Option<(f64,f64)>` | `None`  | A point that will not be drawn, but the beginning of the graph will bend as if it is there. |
| hidden_point_at_end   | `Option<(f64,f64)>` | `None`  | A point that will not be drawn, but the end of the graph will bend as if it is there.       |



```rust
use cubic_spline::{SplineOpts};

let options = SplineOpts::new()
  .tension(0.6)
  .num_of_segments(54)
  // .hidden_point_at_start((1.2, 3.1))
  // .hidden_point_at_end((397.9, 105.5))
  ;



```



##### Enjoy using!

### License

This module is [MIT licensed](./LICENSE).


