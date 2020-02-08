import { getCurvePoints as getCurvePointsOrigin } from './cubic_spline.js'

export function getCurvePoints(pts, opts = {}) {
  return getCurvePointsOrigin(
    pts,
    opts.tension,
    opts.num_of_segments,
  )
}

