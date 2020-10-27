import * as React from 'react'
import Points from './comps/Points'
import Settings from './comps/Settings'
import { CANVAS_H, CANVAS_W, isNum, NOS_DEFAULT, SplineSettings, TENSION_DEFAULT, } from './glob'
import Spline from './Spline'
import PointModel from './Point'

interface IState {
  points: PointModel[]
  settings: SplineSettings
}

const startPoints: PointModel[] = [
  // [300,200],
  // [400,100],
  // [500,200],
  // [400,300],
  // [300,200],


  [ 0, 2 ],
  [ 123, 32 ],
  [ 150, 113 ],
  [ 200, 77 ],
  [ 319, 217 ],
  [ 419, 50 ],
  [ 509, 335 ],
  [ 650, 375 ],
  [ 700, 133 ],
  [ 870, 133 ],
  // [10.0, 200.0], [256.0, 390.0], [512.0, 10.0], [778.0, 200.0]
].map( ( [ x, y ] ) => PointModel.from( { x, y } ) )

class App extends React.Component<{}, IState> {

  spline: Spline
  canvEl = React.createRef<HTMLCanvasElement>()

  constructor( props: {} ) {
    super( props )
    this.spline = new Spline( {
      getCanvasEl: () => this.canvEl.current
    } )
  }

  state: IState = {
    points: startPoints,
    settings: {
      tension: TENSION_DEFAULT,
      custom_tensions: [],
      numOfSegments: NOS_DEFAULT,
      invertYwithHeight: CANVAS_H,
      invertXwithWidth: null,
      // hiddenPointAtEnd: [400 ,130 ],
      // hiddenPointAtStart: [ 400,270 ],
    }
  }

  getFlattenValidPoints( points: PointModel[] ): { validFlattenPoints: number[], customTensions: number[] } {
    const validFlattenPoints: number[] = []
    const customTensions: number[] = []
    points
      .forEach( ( p: PointModel ) => {
        if ( isNum( p.x ) && isNum( p.y ) ) {
          validFlattenPoints.push( p.x, p.y )
          customTensions.push( isNum( p.tension ) ? p.tension : -101 )
        }
      } )

    return { validFlattenPoints, customTensions }
  }

  handlePointsChange = ( pts: Array<PointModel> ) => {
    this.setState( { points: pts } )
  }

  handleSettingsChange = ( newSettings: SplineSettings ) => {
    this.setState( {
      settings: {
        ...this.state.settings,
        ...newSettings
      }
    } )
  }

  componentDidUpdate() {
    this.redraw()
  }

  componentDidMount() {
    this.redraw()
  }


  redraw() {
    const {
      validFlattenPoints,
      customTensions,
    } = this.getFlattenValidPoints( this.state.points )
    this.spline.redraw( validFlattenPoints, { ...this.state.settings, custom_tensions: customTensions } )
  }


  render() {
    return (
      <div className="App">
        <h1 className="title is-1">Demo of cubic_spline</h1>
        <div className="links">
          <a href="https://crates.io/crates/cubic_spline/"><img src="https://img.shields.io/crates/v/cubic_spline.svg"/></a>
          <a href="https://www.npmjs.com/package/cubic-spline-rs"><img
            src="https://img.shields.io/npm/v/cubic-spline-rs.svg"/></a>
          <a href="https://github.com/emgyrz/cubic_spline">Repository</a>
        </div>

        <div className="canvasWrp">
          <canvas ref={ this.canvEl } width={ CANVAS_W } height={ CANVAS_H }>
          </canvas>
        </div>

        <div className="controls columns section">

          <Points points={ this.state.points } onChange={ this.handlePointsChange }/>
          <Settings settings={ this.state.settings } onChange={ this.handleSettingsChange }/>

        </div>
      </div>
    )
  }

}

export default App
