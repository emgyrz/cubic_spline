import * as React from 'react';

type NullNum = number | null

interface IProps {
  value: number
  onChange: (val: number) => void
  step: number
  min: NullNum
}


class Inp extends React.Component<IProps> {
  defaultProps: {
    min: null,
    step: 1
  }

  static parseNum(n: string) {

  }


  handleChange = (ev) => {
    let val = ev.currentTarget.value
    this.props.onChange(parseFloat(val))
  }

  handleWheel = (ev) => {
    ev.stopPropagation()
    ev.preventDefault()
    const { value, min, step, onChange } = this.props
    let val = this.props.value
    if (min !== null) {
      val = val < min ? val : min
    }
    val = ev.deltaY < 0 ? val + step : val - step
    onChange(val)
  }

  render() {
    const { value, onChange, ...otherProps } = this.props
    return (
      <input
        className="input is-primary"
        type="text"
        name="tension"
        placeholder="Tension"
        onChange={this.handleChange}
        value={this.props.value}
        onWheel={this.handleWheel}
        {...otherProps}
      />
    )
  }

}

export default Inp
