
type OptsType = {
  tension?: number,
  num_of_segments?: number,
}

export function getCurvePoints(pts: Float64Array, opts: OptsType): Float64Array;
