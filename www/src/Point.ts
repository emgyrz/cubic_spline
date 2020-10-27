import { getNullableNum, isNum } from './glob'

const nToS = ( n: any ) => isNum( n ) ? n.toString() : ''


type PointData = {
  x: null | number,
  y: null | number,
  tension: null | number,
  segments: null | number,
}

class PointModel {
  private readonly data: PointData

  constructor( arg: PointData ) {
    this.data = {...arg}
  }

  static def = (): PointModel => {
    return new PointModel({x: 10, y: 10, tension: null, segments: null})
  }
  static from = ( part: Partial<PointData>): PointModel => {
    return new PointModel({...PointModel.def().data, ...part })
  }

  set( part: Partial<PointData> ) {
    return new PointModel({...this.data, ...part } )
  }

  get x(): null | number {
    return getNullableNum( this.data.x )
  }

  get y(): null | number {
    return getNullableNum( this.data.y )
  }

  get tension(): null | number {
    return getNullableNum( this.data.tension )
  }

  get x_fmt(): string {
    return nToS( this.data.x )
  }

  get y_fmt(): string {
    return nToS( this.data.y )
  }

  get tension_str(): string {
    return nToS( this.data.tension )
  }

  get segments_str(): string {
    return nToS( this.data.segments )
  }


}

export default PointModel
