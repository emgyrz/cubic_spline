import { getCurvePoints } from "../pkg/index"
import { ValidPointType, SplineSettings } from './glob'

type GetCanvasElFunc = () => null | HTMLCanvasElement
type ContstructorArgs = {
  getCanvasEl: GetCanvasElFunc
}


const COLORS = {
  gradient: [
    'rgba(255,132,82,0.3)',
    'rgba(255,132,82,0.1)'
  ],
  stroke: 'rgba(255,132,82,1)'
}


export default class Spline {
  private getCanvasEl: GetCanvasElFunc

  constructor ({ getCanvasEl }: ContstructorArgs) {
    this.getCanvasEl = getCanvasEl
  }

  private get el(): null | HTMLCanvasElement {
    return this.getCanvasEl()
  }
  private get ctx(): null | CanvasRenderingContext2D {
    const { el } = this
    if (el === null) return null
    return el.getContext('2d')
  }

  public draw(pts: number[], settings: SplineSettings) {
    const { ctx } = this
    if (ctx === null) return

    const curvePoints = this.getCurvePoints(pts, settings)
    this.drawLine(curvePoints)
    this.drawShadow(curvePoints)
    this.drawPoints(pts)


  }

  public clear() {
    const { ctx, el } = this
    if (el === null || ctx === null) return
    ctx.clearRect(0, 0, el.width, el.height)
  }

  public redraw(pts: number[], settings: SplineSettings) {
    this.clear()
    this.draw(pts, settings)
  }

  private getCurvePoints(pts: number[], settings: SplineSettings): Float64Array {
    return getCurvePoints(Float64Array.from(pts), {
      tension: settings.tension,
      num_of_segments: settings.numOfSegments,
    })
  }

  private drawLine(curvePoints: Float64Array) {
    const ctx = this.ctx!

    ctx.beginPath()
    ctx.lineWidth = 3
    ctx.strokeStyle = COLORS.stroke

    ctx.moveTo(curvePoints[0], curvePoints[1])
    const length = curvePoints.length - 1
    for (let i = 2; i < length; i += 2) {
      ctx.lineTo(curvePoints[i], curvePoints[i + 1])
    }

    ctx.stroke()
    ctx.closePath()
  }

  private drawShadow(pts: Float64Array) {
    const ctx = this.ctx!
    const canvHeight = ctx.canvas.clientHeight

    const prePoint: ValidPointType = [pts[0], canvHeight];
    const postPoint: ValidPointType = [pts[pts.length - 2], canvHeight];


    let maxY = pts[1]
    const ptsLength = pts.length - 1


    ctx.beginPath()
    ctx.moveTo(...prePoint)
    for (let i = 0; i < ptsLength; i += 2) {
      const x = pts[i]
      const y = pts[i + 1]
      ctx.lineTo(x, y)
      if (y < maxY) maxY = y
    }
    ctx.lineTo(...postPoint)

    const gradient = ctx.createLinearGradient(0, maxY, 0, canvHeight)
    gradient.addColorStop(0, COLORS.gradient[0])
    gradient.addColorStop(1, COLORS.gradient[1])

    ctx.strokeStyle = 'transparent'
    ctx.lineWidth = 0
    ctx.fillStyle = gradient

    ctx.fill()
    ctx.stroke()
    ctx.closePath()
  }


  private drawPoints(pts: number[]) {
    const ctx = this.ctx!

    ctx.strokeStyle = "#8259af";
    ctx.lineWidth = 2;
    for (let i = 0; i < pts.length - 1; i += 2) {
      ctx.beginPath()
      ctx.arc(pts[i], pts[i + 1], 4, 0, 2 * Math.PI);
      ctx.stroke();
    }

  }

}
