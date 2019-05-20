import * as React from 'react';

type NullNum = number | null

interface IProps {
  x: NullNum
  y: NullNum
  idx: number
  onChange: (arg: { idx: number, x: NullNum, y: NullNum }) => void,
  onDel: (idx: number) => void
}


class Point extends React.Component<IProps> {

  static toVal(n: NullNum): string {
    return typeof n === 'number' ? n.toString() : ''
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
    const { toVal } = Point
    const { idx, x, y } = this.props
    return (
      <div className="field is-horizontal has-addons">
        <div className="field-label is-normal">
          <label className="label">{idx}.</label>
        </div>
        <div className="field-body" style={{ alignItems: 'center' }}>
          <div className="field" style={{ maxWidth: 100 }}>
            <p className="control is-expanded">
              <input
                className="input is-success"
                type="text"
                placeholder="X"
                value={toVal(x)}
                data-plane="x"
                onChange={this.handleChange}
              />
            </p>
          </div>
          <div className="field" style={{ maxWidth: 100 }}>
            <p className="control is-expanded">
              <input
                className="input is-success"
                type="text"
                placeholder="Y"
                value={toVal(y)}
                data-plane="y"
                onChange={this.handleChange}
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
