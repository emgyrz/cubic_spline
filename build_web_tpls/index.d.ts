
export type OptsType = {
  tension?: number,
  num_of_segments?: number,
  invert_x_with_width?: number,
  invert_y_with_height?: number,
}

export function getCurvePoints(pts: Float64Array, opts: OptsType): Float64Array;
