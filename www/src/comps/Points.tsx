import * as React from 'react';
import Point from './Point'
import PointModel from '../Point'

interface IProps {
  points: Array<PointModel>;
  onChange: (pts: Array<PointModel>) => void;
}


class Points extends React.Component<IProps> {
  handleChange = (res: { idx: number, point: PointModel }) => {
    const pts = this.props.points.slice(0)
    pts[res.idx] = res.point
    this.props.onChange(pts)
  }

  handleAddClick = () => {
    const pts = this.props.points.slice(0)
    let last  = PointModel.def()
    if (pts.length !== 0) {
      last = pts[pts.length - 1]
    }
    pts.push(last.set({x: last.x, y: last.y}))
    this.props.onChange(pts)
  }

  handleDel = (idx: number) => {
    const pts = this.props.points.slice(0)
    pts.splice(idx, 1)
    this.props.onChange(pts)
  }

  render() {
    const { points } = this.props
    const pts = points.map((p, idx) => (
      <Point
        key={idx}
        idx={idx}
        point={p}
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
            <div className="field" style={{ maxWidth: 50 }}>
                x
            </div>
            <div className="field" style={{ maxWidth: 50 }}>
                y
            </div>
            <div className="field" style={{ maxWidth: 50 }}>
              custom tension
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
