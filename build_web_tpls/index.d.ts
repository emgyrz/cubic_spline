
export type OptsType = {
  tension?: number,
  custom_tensions?: number[],
  num_of_segments?: number,
  invert_x_with_width?: number,
  invert_y_with_height?: number,
  hidden_point_at_start?: number[],
  hidden_point_at_end?: number[],
}

export function getCurvePoints(pts: Float64Array, opts: OptsType): Float64Array;
