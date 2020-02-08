import * as React from 'react';
import Point from './Point'
import { NullNum } from '../glob'

interface IProps {
  points: Array<[NullNum, NullNum]>;
  onChange: (pts: Array<[NullNum, NullNum]>) => void;
}


class Points extends React.Component<IProps> {
  handleChange = (res: { idx: number, x: NullNum, y: NullNum }) => {
    const pts = this.props.points.slice(0)
    pts[res.idx] = [res.x, res.y]
    this.props.onChange(pts)
  }

  handleAddClick = () => {
    const pts = this.props.points.slice(0)
    let last: [NullNum, NullNum] = [10, 10]
    if (pts.length !== 0) {
      last = pts[pts.length - 1]
    }
    pts.push([last[0], last[1]])
    this.props.onChange(pts)
  }

  handleDel = (idx: number) => {
    const pts = this.props.points.slice(0)
    pts.splice(idx, 1)
    this.props.onChange(pts)
  }

  render() {
    const { points } = this.props
    const pts = points.map(([x, y], idx) => (
      <Point
        key={idx}
        idx={idx}
        x={x}
        y={y}
        onChange={this.handleChange}
        onDel={this.handleDel}
      />
    ))
    return (
      <div className="points column is-half">
        <h2 className="title">Points</h2>
        <h6 className="subtitle is-7">Scroll above input to change value</h6>
        <div className="field is-horizontal has-addons">
          <div className="field-label is-normal">
          </div>
          <div className="field-body" style={{ alignItems: 'center' }}>
            <div className="field" style={{ maxWidth: 100 }}>
                x
            </div>
            <div className="field" style={{ maxWidth: 100 }}>
                y
            </div>
          </div>
        </div>
        {pts}
        <button
          className="button is-primary is-outlined addBtn"
          onClick={this.handleAddClick}
        >
          Add
        </button>
      </div>
    )

  }
}

export default Points
