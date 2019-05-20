

import { getCurvePoints } from "../pkg";



const tension = 0.5
const numOfSegments = 16



function drawSpline(ctx, ptsa, attrs, showPoints) {


  applyAttrs(ctx, attrs)

  ctx.beginPath()

  const curvePoints = getCurvePoints(ptsa, tension, numOfSegments)
  console.log('curvePoints: ', curvePoints);

  drawLines(ctx, curvePoints)


  ctx.stroke()
  ctx.closePath()


  // drawShadow(ctx, curvePoints)

  if (showPoints) {
    ctx.beginPath()
    ctx.strokeStyle = "#7B1FA2";
    // ctx.fillStyle = "orange";
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




// function getCurvePoints(pts) {


//   let _pts = []


//   const res = []


//   let x; let y


//   let t1x; let t2x; let t1y; let t2y


//   let c1; let c2; let c3; let c4


//   let st;
//   let t

//   _pts = pts.slice(0)

//   _pts.unshift(pts[1])
//   _pts.unshift(pts[0])
//   _pts.push(pts[pts.length - 2])
//   _pts.push(pts[pts.length - 1])



//   for (let i = 2; i < (_pts.length - 4); i += 2) {
//     for (t = 0; t <= numOfSegments; t++) {

//       t1x = (_pts[i + 2] - _pts[i - 2]) * tension
//       t2x = (_pts[i + 4] - _pts[i]) * tension


//       t1y = (_pts[i + 3] - _pts[i - 1]) * tension
//       t2y = (_pts[i + 5] - _pts[i + 1]) * tension

//       st = t / numOfSegments

//       c1 = 2 * Math.pow(st, 3) - 3 * Math.pow(st, 2) + 1
//       c2 = 2 * Math.pow(st, 3) * -1 + 3 * Math.pow(st, 2)
//       c3 = Math.pow(st, 3) - 2 * Math.pow(st, 2) + st
//       c4 = Math.pow(st, 3) - Math.pow(st, 2)


//       x = c1 * _pts[i] + c2 * _pts[i + 2] + c3 * t1x + c4 * t2x
//       y = c1 * _pts[i + 1] + c2 * _pts[i + 3] + c3 * t1y + c4 * t2y


//       // if (lastX && x < lastX) {
//       //   x = lastX
//       // }
//       res.push(x)
//       res.push(y + 1)

//     }
//   }

//   return res
// }

function drawLines(ctx, pts) {
  ctx.moveTo(pts[0], pts[1])
  const length = pts.length - 1
  for (let i = 2; i < length; i += 2) {
    ctx.lineTo(pts[i], pts[i + 1])
  }
}

export default drawSpline

