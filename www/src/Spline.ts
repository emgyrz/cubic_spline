import { getCurvePoints, OptsType } from "../pkg/index"
import { ValidPointType, SplineSettings, isNum } from './glob'

type GetCanvasElFunc = () => null | HTMLCanvasElement
type ContstructorArgs = {
  getCanvasEl: GetCanvasElFunc
}


const COLORS = {
  gradient: [
    'rgba(139,195,74,0.3)',
    'rgba(139,195,74,0.1)'
  ],
  stroke: 'rgba(139,195,74,1)',
  point: '#fff176'
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

    if (
      !isNum(settings.invertXwithWidth) &&
      isNum(settings.invertYwithHeight)
    ) {
      this.drawPoints(pts)
    }
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

    const opts: OptsType = {
      tension: settings.tension,
      num_of_segments: settings.numOfSegments,
      custom_tensions: settings.custom_tensions,
    }
    if (isNum(settings.invertXwithWidth)) {
      opts.invert_x_with_width = settings.invertXwithWidth
    }
    if (isNum(settings.invertYwithHeight)) {
      opts.invert_y_with_height = settings.invertYwithHeight
    }
    if (Array.isArray(settings.hiddenPointAtStart)) {
      opts.hidden_point_at_start = settings.hiddenPointAtStart
    }
    if (Array.isArray(settings.hiddenPointAtEnd)) {
      opts.hidden_point_at_end = settings.hiddenPointAtEnd
    }


    return getCurvePoints(Float64Array.from(pts), opts)


    // @ts-ignore
    // return [[0.0, 2.0], [4.4849853515625, 2.9576416015625], [10.1455078125, 3.9658203125], [16.8211669921875, 5.0399169921875], [24.3515625, 6.1953125], [32.5762939453125, 7.4473876953125], [41.3349609375, 8.8115234375], [50.4671630859375, 10.3031005859375], [59.8125, 11.9375], [69.2105712890625, 13.7301025390625], [78.5009765625, 15.6962890625], [87.5233154296875, 17.8514404296875], [96.1171875, 20.2109375], [104.1221923828125, 22.7901611328125], [111.3779296875, 25.6044921875], [117.7239990234375, 28.6693115234375], [123.0, 32.0], [127.2821044921875, 35.8759765625], [130.8115234375, 40.484375], [133.6754150390625, 45.7021484375], [135.9609375, 51.40625], [137.7552490234375, 57.4736328125], [139.1455078125, 63.78125], [140.2188720703125, 70.2060546875], [141.0625, 76.625], [141.7635498046875, 82.9150390625], [142.4091796875, 88.953125], [143.0865478515625, 94.6162109375], [143.8828125, 99.78125], [144.8851318359375, 104.3251953125], [146.1806640625, 108.125], [147.8565673828125, 111.0576171875], [150.0, 113.0], [152.366943359375, 113.6412353515625], [154.677734375, 112.8955078125], [156.966064453125, 110.9774169921875], [159.265625, 108.1015625], [161.610107421875, 104.4825439453125], [164.033203125, 100.3349609375],[166.568603515625, 95.8734130859375], [169.25, 91.3125], [172.111083984375, 86.8668212890625],[175.185546875, 82.7509765625], [178.507080078125, 79.1795654296875], [182.109375, 76.3671875], [186.026123046875, 74.5284423828125], [190.291015625, 73.8779296875], [194.937744140625, 74.6302490234375], [200.0, 77.0], [205.5771484375, 81.4781494140625], [211.703125, 88.1767578125], [218.3134765625, 96.7420654296875], [225.34375, 106.8203125], [232.7294921875, 118.0577392578125], [240.40625, 130.1005859375], [248.3095703125, 142.5950927734375], [256.375, 155.1875], [264.5380859375, 167.5240478515625], [272.734375, 179.2509765625], [280.8994140625, 190.0145263671875], [288.96875, 199.4609375], [296.8779296875, 207.2364501953125], [304.5625, 212.9873046875],[311.9580078125, 216.3597412109375], [319.0, 217.0], [325.7901611328125, 214.1668701171875], [332.4775390625, 207.7255859375], [339.0687255859375, 198.2320556640625], [345.5703125, 186.2421875], [351.9888916015625, 172.3118896484375], [358.3310546875, 156.9970703125], [364.6033935546875, 140.8536376953125], [370.8125, 124.4375], [376.9649658203125, 108.3045654296875], [383.0673828125, 93.0107421875], [389.1263427734375, 79.1119384765625], [395.1484375, 67.1640625], [401.1402587890625, 57.7230224609375], [407.1083984375, 51.3447265625], [413.0594482421875, 48.5850830078125], [419.0, 50.0], [424.8062744140625, 55.8465576171875], [430.3798828125, 65.6708984375], [435.7655029296875, 78.9625244140625], [441.0078125, 95.2109375], [446.1514892578125, 113.9056396484375], [451.2412109375, 134.5361328125], [456.3216552734375, 156.5919189453125], [461.4375, 179.5625], [466.6334228515625, 202.9373779296875], [471.9541015625, 226.2060546875], [477.4442138671875, 248.8580322265625], [483.1484375, 270.3828125], [489.1114501953125, 290.2698974609375], [495.3779296875, 308.0087890625], [501.9925537109375, 323.0889892578125], [509.0, 335.0], [516.578369140625, 344.7454833984375], [524.806640625, 353.6513671875], [533.580810546875, 361.6905517578125], [542.796875, 368.8359375], [552.350830078125, 375.0604248046875], [562.138671875, 380.3369140625], [572.056396484375, 384.6383056640625], [582.0, 387.9375], [591.865478515625, 390.2073974609375], [601.548828125, 391.4208984375], [610.946044921875, 391.5509033203125], [619.953125, 390.5703125], [628.466064453125, 388.4520263671875], [636.380859375, 385.1689453125], [643.593505859375, 380.6939697265625], [650.0, 375.0], [655.4046630859375, 367.17724609375], [659.7841796875, 356.58984375], [663.2930908203125, 343.62158203125], [666.0859375, 328.65625], [668.3172607421875, 312.07763671875], [670.1416015625, 294.26953125], [671.7135009765625, 275.61572265625], [673.1875, 256.5], [674.7181396484375, 237.30615234375], [676.4599609375, 218.41796875], [678.5675048828125, 200.21923828125], [681.1953125, 183.09375], [684.4979248046875,167.42529296875], [688.6298828125, 153.59765625], [693.7457275390625, 141.99462890625], [700.0, 133.0], [707.640380859375, 126.353271484375], [716.669921875, 121.419921875], [726.876220703125, 118.022705078125], [738.046875, 115.984375], [749.969482421875, 115.127685546875], [762.431640625, 115.275390625], [775.220947265625, 116.250244140625], [788.125, 117.875], [800.931396484375, 119.972412109375], [813.427734375, 122.365234375], [825.401611328125, 124.876220703125], [836.640625, 127.328125], [846.932373046875, 129.543701171875], [856.064453125, 131.345703125],[863.824462890625, 132.556884765625], [870.0, 133.0]].flat()
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


  private invertPtsY(pts: number[], h: number): number[] {
    return pts.map((xOrY: number, ind: number): number => {
      if (ind % 2 !== 0) {
        const y = xOrY
        return h - y
      }
      return xOrY
    })
  }

  private drawPoints(pts: number[]) {
    const ctx = this.ctx!
    const inverted = this.invertPtsY(pts, ctx.canvas.clientHeight)
    ctx.strokeStyle = COLORS.point;
    ctx.lineWidth = 2;
    for (let i = 0; i < inverted.length - 1; i += 2) {
      ctx.beginPath()
      ctx.arc(inverted[i], inverted[i + 1], 4, 0, 2 * Math.PI);
      ctx.stroke();
    }

  }

}
