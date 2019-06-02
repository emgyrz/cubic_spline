
type OptsType = {
  tension?: number,
  num_of_segments?: number,
  disallow_x_stepping_back?: boolean,
}

export function getCurvePoints(pts: Float64Array, opts: OptsType): Float64Array;
