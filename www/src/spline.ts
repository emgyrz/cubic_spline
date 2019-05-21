

import { getCurvePoints } from "../pkg";



const tension = 0.5
const numOfSegments = 16

function drawSpline(ctx, ptsa, attrs, showPoints) {


  applyAttrs(ctx, attrs)

  ctx.beginPath()

  const curvePoints = getCurvePoints(ptsa, tension, numOfSegments)

  drawLines(ctx, curvePoints)

  ctx.stroke()
  ctx.closePath()


  // drawShadow(ctx, curvePoints)

  if (showPoints) {
    ctx.beginPath()
    ctx.strokeStyle = "#7B1FA2";
    ctx.lineWidth = 7;
    for (let i = 0; i < ptsa.length - 1; i += 2) ctx.rect(ptsa[i] - 2, ptsa[i + 1] - 2, 4, 4)
    ctx.stroke();
  }

  return curvePoints

}



function drawShadow(ctx, points) {
  ctx.beginPath()
  const ctxHeight = ctx.canvas.clientHeight

  points.unshift(points[0], ctxHeight)
  points.push(points[points.length - 2], ctxHeight)



  let topY = points[1]


  ctx.moveTo(points[0], points[1])

  const ptsLength = points.length - 1
  for (let i = 2; i < ptsLength; i += 2) {
    const x = points[i]

    const y = points[i + 1]
    ctx.lineTo(x, y)
    if (y < topY) topY = y
  }

  const gradient = ctx.createLinearGradient(0, topY, 0, ctxHeight)
  gradient.addColorStop(0, 'rgba(100,130,100,0.09)')
  gradient.addColorStop(1, 'rgba(100,130,100,0.04)')


  ctx.strokeStyle = 'transparent'
  ctx.lineWidth = 0

  ctx.fillStyle = gradient
  ctx.fill()
  ctx.stroke()
  ctx.closePath()
}



function applyAttrs(ctx, attrs) {
  if (typeof attrs === 'object' && attrs !== null) {
    for (const key in attrs) {
      if (attrs.hasOwnProperty(key)) {
        ctx[key] = attrs[key]
      }
    }
  }
}

function drawLines(ctx, pts) {
  ctx.moveTo(pts[0], pts[1])
  const length = pts.length - 1
  for (let i = 2; i < length; i += 2) {
    ctx.lineTo(pts[i], pts[i + 1])
  }
}

export default drawSpline

