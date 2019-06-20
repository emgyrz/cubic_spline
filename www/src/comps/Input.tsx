import * as React from 'react';

type NullNum = number | null

interface IProps extends React.HTMLAttributes<HTMLInputElement> {
  value: NullNum;
  onInpChange: (val: NullNum) => void;
  step?: number;
  min?: NullNum;
}

class Inp extends React.Component<IProps, {}> {
  static defaultProps = {
    min: null,
    step: 1
  }

  inp = React.createRef<HTMLInputElement>()


  componentDidMount() {
    this.inp.current.addEventListener("wheel", this.handleWheel)
  }

  componentWillUnmount() {
    this.inp.current.removeEventListener("wheel", this.handleWheel)
  }


  handleChange = (ev) => {
    let val = ev.currentTarget.value
    val = val === '' ? null : parseFloat(val)
    val = Number.isNaN(val) ? null : val
    this.props.onInpChange(val)
  }

  handleWheel = (ev) => {
    ev.stopPropagation()
    ev.preventDefault()
    const { value, min, step, onInpChange } = this.props
    let val = this.props.value
    if (val === null) return
    if (min !== null) {
      val = val < min ? val : min
    }
    val = ev.deltaY < 0 ? val + step : val - step
    onInpChange(val)
  }

  render() {
    const { value, onInpChange, ...otherProps } = this.props
    let val = value === null ? '' : value
    return (
      <input
        {...otherProps}
        type="text"
        onChange={this.handleChange}
        value={val}
        ref={this.inp}
        style={{ cursor: 'ns-resize' }}
      />
    )
  }

}

export default Inp
