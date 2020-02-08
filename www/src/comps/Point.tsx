import * as React from 'react';
import Inp from './Input'
import { NullNum } from '../glob'

interface IProps {
  x: NullNum
  y: NullNum
  idx: number
  onChange: (arg: { idx: number, x: NullNum, y: NullNum }) => void,
  onDel: (idx: number) => void
}


class Point extends React.Component<IProps> {

  handleXChange = ( x: NullNum ) => {
    this.emitChange( 'x', x )
  }

  handleYChange = ( y: NullNum ) => {
    this.emitChange( 'y', y )
  }

  emitChange( key: 'x' | 'y', val: NullNum ) {
    const res = {
      x: this.props.x,
      y: this.props.y,
      idx: this.props.idx
    }
    res[key] = val
    this.props.onChange(res)
  }


  handleChange = (ev: React.ChangeEvent<HTMLInputElement>) => {
    const trg = ev.target
    const { plane } = trg.dataset
    if (plane !== 'x' && plane !== 'y') return
    const res = {
      x: this.props.x,
      y: this.props.y,
      idx: this.props.idx
    }
    let val: NullNum = parseFloat(ev.target.value)
    val = Number.isNaN(val) ? null : val
    res[plane] = val
    this.props.onChange(res)
  }

  handleDelClick = () => {
    this.props.onDel(this.props.idx)
  }

  render() {
    const { idx, x, y } = this.props
    return (
      <div className="field is-horizontal has-addons">
        <div className="field-label is-normal">
          <label className="label">{idx}.</label>
        </div>
        <div className="field-body" style={{ alignItems: 'center' }}>
          <div className="field" style={{ maxWidth: 100 }}>
            <p className="control is-expanded">
              <Inp
                className="input is-success"
                value={x}
                onInpChange={this.handleXChange}
                step={5}
              />
            </p>
          </div>
          <div className="field" style={{ maxWidth: 100 }}>
            <p className="control is-expanded">
              <Inp
                className="input is-success"
                value={y}
                onInpChange={this.handleYChange}
                step={5}
              />
            </p>
          </div>
          <button className="delete" onClick={this.handleDelClick}></button>
        </div>
      </div>
    )
  }
}

export default Point;
