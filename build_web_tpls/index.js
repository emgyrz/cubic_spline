import { getCurvePoints as getCurvePointsOrigin } from './cubic_spline.js'

export function getCurvePoints(pts, opts = {}) {
  return getCurvePointsOrigin(
    pts,
    opts.tension,
    opts.num_of_segments,
    opts.invert_x_with_width,
    opts.invert_y_with_height,
  )
}

