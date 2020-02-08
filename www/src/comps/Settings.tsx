import * as React from 'react';

import {
  isNum,
  castNum,
  NullNum,
  PointType,
  SplineSettings,
  TENSION_DEFAULT,
} from '../glob'

interface IProps {
  onChange: (s: SplineSettings) => void,
  settings: SplineSettings
}

type ChangeEv = React.ChangeEvent<HTMLInputElement>

export default class Points extends React.Component<IProps> {

  emitChange(partialSettings: Partial<SplineSettings>) {
    this.props.onChange({
      ...this.props.settings,
      ...partialSettings
    })
  }

  handleChange = (ev: React.ChangeEvent<HTMLInputElement>) => {
    const trg = ev.target
    const { name, value } = trg
    const upd = this.emitChange
    switch (name) {
      case 'tension': {
        const val = parseFloat(value)
        if (isNum(val)) {
          this.emitChange({ [name]: val })
        }
        break
      }
      case 'numOfSegments': {
        const val = parseInt(value)
        if (isNum(val)) {
          this.emitChange({ [name]: val })
        }
        break
      }
      default:
        console.warn('invalid input name')
        break
    }
  }



  render() {
    const { settings } = this.props

    return (
      <div className="settings column is-half">
        <h2 className="title">Settings</h2>

        <div className="field range is-horizontal">
          <div className="inpWrp">
            <label className="label">tension:</label>
            <div className="control">
              <input
                name="tension"
                className="slider has-output is-fullwidth"
                min="-3"
                max="3"
                value={settings.tension}
                step="0.1"
                type="range"
                onChange={this.handleChange}
              />
            </div>
          </div>
          <b className="val">{settings.tension.toFixed(1)}</b>
        </div>

        <div className="field range is-horizontal">
          <div className="inpWrp">
            <label className="label">num_of_segments:</label>
            <div className="control">
              <input
                name="numOfSegments"
                className="slider has-output is-fullwidth"
                min="1"
                max="50"
                value={settings.numOfSegments}
                step="1"
                type="range"
                onChange={this.handleChange}
              />
            </div>
          </div>
          <b className="val">{settings.numOfSegments}</b>
        </div>

      </div>

    )
  }

}
