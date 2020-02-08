
export type NullNum = number | null
export type PointType = [NullNum, NullNum]
export type ValidPointType = [number, number]
export type SplineSettings = {
  tension: number,
  numOfSegments: number,
  disallowXsteppingBack: boolean,
}


export const TENSION_DEFAULT = 0.5
export const NOS_DEFAULT = 16

export function isNum(n: any): n is number {
  return typeof n === 'number' && !Number.isNaN(n)
}

export function castNum(n: any, def?: number): number {
  return isNum(n) ? n : ( isNum(def) ? def : 0 )
}


// export function filterInvalidPoints() {

// }
