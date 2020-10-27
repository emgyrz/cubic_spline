import { getCurvePoints as getCurvePointsOrigin } from './cubic_spline.js'

export function getCurvePoints(pts, opts = {}) {
  return getCurvePointsOrigin(
    pts,
    opts.num_of_segments,
    opts.tension,
    opts.custom_tensions,
    opts.invert_x_with_width,
    opts.invert_y_with_height,
    opts.hidden_point_at_start,
    opts.hidden_point_at_end,
  )
}

