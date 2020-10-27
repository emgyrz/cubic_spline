import * as React from 'react'
import Inp from './Input'
import { getNullableNum, isNum, NullNum } from '../glob'
import PointModel from '../Point'

interface IProps {
  point: PointModel,
  idx: number
  onChange: ( arg: { idx: number, point: PointModel } ) => void,
  onDel: ( idx: number ) => void
}


class Point extends React.Component<IProps> {

  handleXChange = ( x: NullNum ) => {
    this.emitChange( 'x', x )
  }

  handleYChange = ( y: NullNum ) => {
    this.emitChange( 'y', y )
  }

  handleTensionChange = ( tension: NullNum ) => {
    const t = isNum(tension) ? parseFloat(tension.toFixed(1)) : null
    this.emitChange( 'tension', t )
  }

  emitChange( key: 'x' | 'y' | 'tension', val: NullNum ) {
    this.props.onChange( {
      point: this.props.point.set( { [ key ]: val } ),
      idx: this.props.idx
    } )
  }

  handleDelClick = () => {
    this.props.onDel( this.props.idx )
  }

  render() {
    const { idx, point } = this.props
    return (
      <div className="field is-horizontal has-addons">
        <div className="field-label is-normal">
          <label className="label">{ idx }.</label>
        </div>
        <div className="field-body" style={ { alignItems: 'center' } }>
          <div className="field" style={ { maxWidth: 50 } }>
            <p className="control is-expanded">
              <Inp
                className="input is-success"
                value={ point.x }
                onInpChange={ this.handleXChange }
                step={ 5 }
              />
            </p>
          </div>
          <div className="field" style={ { maxWidth: 50 } }>
            <p className="control is-expanded">
              <Inp
                className="input is-success"
                value={ point.y }
                onInpChange={ this.handleYChange }
                step={ 5 }
              />
            </p>
          </div>
          <div className="field" style={ { maxWidth: 50 } }>
            <p className="control is-expanded">
              <Inp
                className="input is-success"
                value={ point.tension }
                onInpChange={ this.handleTensionChange }
                step={ 0.1 }
              />
            </p>
          </div>
          <button className="delete" onClick={ this.handleDelClick }></button>
        </div>
      </div>
    )
  }
}

export default Point
