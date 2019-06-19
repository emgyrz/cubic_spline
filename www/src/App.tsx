import * as React from 'react';
import Points from './comps/Points'
import Inp from './comps/Input'
import drawSpline from './spline'

type NullNum = number | null
type PointType = [NullNum, NullNum]
interface IState {
  points: PointType[];
  tension: number;
  numOfSegments: number;
  disallowXsteppingBack: boolean;
}

const TENSION_DEFAULT = 0.5
const NOS_DEFAULT = 16

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
    points: startPoints,
    tension: 0.5,
    numOfSegments: 16,
    disallowXsteppingBack: false,
  }

  canv = React.createRef<HTMLCanvasElement>()
  ctx: CanvasRenderingContext2D | null = null

  preparePoints(points: PointType[]) {
    const canv = this.canv.current
    if (canv === null) return
    const h = canv.height

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


  handleTensionChange = ( tension: NullNum ) => {
    this.setState( { tension })
  }


  handleNumOfSegmentsChange = ( num: NullNum ) => {
    if ( num !== null && num < 1 ) return
    this.setState( { numOfSegments: num })
  }

  handleDisallowXsteppingBackChange = ( ev ) => {
    this.setState({disallowXsteppingBack: ev.currentTarget.checked })
  }

  componentDidUpdate() {
    this.redraw()
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

  redraw() {
    const pts = this.preparePoints(this.state.points)

    this.clearCanv()
    const ctx = this.getCtx()
    const { tension, numOfSegments, disallowXsteppingBack } = this.state
    drawSpline(ctx, pts, {
      lineWidth: 3,
      strokeStyle: '#7B1FA2',
      tension: tension === null ? TENSION_DEFAULT : tension,
      numOfSegments: numOfSegments === null ? NOS_DEFAULT : numOfSegments,
      disallowXsteppingBack,
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

            {/* <div className="field">
              <label className="label">Color</label>
              <div className="control">
                <input
                  className="input is-primary"
                  type="text"
                  placeholder="Color"
                  disabled
                  value="*"
                />
              </div>
            </div> */}

            <div className="field is-horizontal">
              <label className="label">tension:</label>
              <div className="control">
                <Inp
                  className="input is-primary"
                  step={0.1}
                  placeholder="Tension"
                  value={this.state.tension}
                  onInpChange={this.handleTensionChange}
                />
              </div>
            </div>

            <div className="field is-horizontal">
              <label className="label">num_of_segments:</label>
              <div className="control">
                <Inp
                  className="input is-primary"
                  value={this.state.numOfSegments}
                  onInpChange={this.handleNumOfSegmentsChange}
                />
              </div>
            </div>

            <div className="field is-horizontal">
              <label className="label">disallow_x_stepping_back:</label>
              <div className="control">
                <label className="checkbox">
                  <input
                    type="checkbox"
                    checked={this.state.disallowXsteppingBack}
                    onChange={this.handleDisallowXsteppingBackChange}
                  />
                </label>
              </div>
            </div>



          </div>

        </div>
      </div>
    );
  }

}

export default App;
