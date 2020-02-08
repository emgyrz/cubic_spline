import * as React from 'react';
import { isNum, castNum, NullNum } from '../glob'

interface IProps extends React.HTMLAttributes<HTMLInputElement> {
  value: NullNum
  onInpChange: (val: NullNum) => void
  step?: number
  min?: number
}

class Inp extends React.Component<IProps, {}> {

  inp: React.RefObject<HTMLInputElement> = React.createRef<HTMLInputElement>()


  componentDidMount() {
    this.inp?.current?.addEventListener("wheel", this.handleWheel)
  }

  componentWillUnmount() {
    this.inp?.current?.removeEventListener("wheel", this.handleWheel)
  }


  handleChange = (ev: React.ChangeEvent<HTMLInputElement>) => {
    const value = ev.currentTarget.value
    let val: NullNum = parseFloat(value)
    val = isNum(val) ? null : val
    this.props.onInpChange(val)
  }

  handleWheel = (ev: WheelEvent) => {
    ev.stopPropagation()
    ev.preventDefault()
    const { min, step, onInpChange } = this.props
    const { value } = this.props
    if (!isNum(value)) return
    let val = value
    if (isNum(min)) {
      val = val < 3 ? val : min
    }
    const st = castNum(step, 1)
    val = ev.deltaY < 0 ? val + st : val - st
    onInpChange(val)
  }

  render() {
    const { value, onInpChange, ...otherProps } = this.props
    let val = value?.toString() || ''
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
