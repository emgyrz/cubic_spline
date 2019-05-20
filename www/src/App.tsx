import * as React from 'react';
import Points from './comps/Points'
import drawSpline from './spline'

type NullNum = number | null
type PointType = [NullNum, NullNum]
interface IState {
  points: PointType[];
}

const startPoints: PointType[] = [
  [0, 2],
  [123, 32],
  [150, 113],
  [200, 77],
  [319, 217],
  [419, 50],
  [509, 335],
  [650, 375],
  [700, 133],
  [870, 133],
]

class App extends React.Component<{}, IState> {
  state: IState = {
    points: startPoints
  }

  canv = React.createRef<HTMLCanvasElement>()
  ctx: CanvasRenderingContext2D | null = null

  preparePoints(points: PointType[]) {
    const canv = this.canv.current
    if (canv === null) return
    const h = canv.height
    console.log('h: ', h);

    const validPoints: number[] = []

    points.forEach(([x, y]: PointType) => {
      if (x !== null && y !== null) {
        validPoints.push(x, h - y)
      }
    })

    return validPoints
  }

  handlePointsChange = (pts: Array<[NullNum, NullNum]>) => {
    this.setState({ points: pts })
  }

  componentWillUpdate(prevState: IState, nextState: IState) {
    this.redraw(nextState.points)
  }

  componentDidMount() {
    this.redraw()
  }

  getCtx(): CanvasRenderingContext2D {
    if (this.ctx !== null) return this.ctx
    const canv = this.canv.current

    if (canv !== null) {
      const ctx = canv.getContext('2d')
      if (ctx !== null) {
        this.ctx = ctx
        return ctx
      }
    }

    return new CanvasRenderingContext2D()
  }

  clearCanv() {
    const canv = this.canv.current
    if (canv === null) return
    const ctx = this.getCtx()
    ctx.clearRect(0, 0, canv.width, canv.height)
  }

  redraw(points?: Array<[NullNum, NullNum]>) {
    const pts = this.preparePoints(points || this.state.points)

    this.clearCanv()
    const ctx = this.getCtx()
    drawSpline(ctx, pts, {
      lineWidth: 3,
      strokeStyle: '#7B1FA2',
      // fillStyle: "#E64A19"
    }, true)

  }

  render() {
    return (
      <div className="App">
        <h1 className="title is-1">Demo of cubic_spline</h1>
        <div className="links">
          <a href="https://crates.io/crates/cubic_spline/"><img src="https://img.shields.io/crates/v/cubic_spline.svg" /></a>
          <a href="https://github.com/emgyrz/cubic_spline">Repository</a>
        </div>

        <div className="canvasWrp">
          <canvas ref={this.canv} width="900" height="400">
          </canvas>
        </div>

        <div className="controls columns section">

          <Points points={this.state.points} onChange={this.handlePointsChange} />

          <div className="settings column is-half">
            <h2 className="title">Settings</h2>

            <div className="field">
              <label className="label">Color</label>
              <div className="control">
                <input
                  className="input is-primary"
                  type="text"
                  placeholder="Color"
                  disabled
                  value="green"
                />
              </div>
            </div>

            <div className="field">
              <label className="label">Tension</label>
              <div className="control">
                <input
                  className="input is-primary"
                  type="text"
                  placeholder="Tension"
                  disabled
                  value="0.5"
                />
              </div>
            </div>

            <div className="field">
              <label className="label">Number of segments</label>
              <div className="control">
                <input
                  className="input is-primary"
                  type="text"
                  placeholder="Number of segments"
                  disabled
                  value={16}
                />
              </div>
            </div>

          </div>

        </div>
      </div>
    );
  }

}

export default App;
