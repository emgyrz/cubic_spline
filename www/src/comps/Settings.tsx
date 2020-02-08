import * as React from 'react';

import {
  isNum,
  SplineSettings,
  CANVAS_W,
  CANVAS_H,
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
      case 'invertXwithWidth': {
        this.emitChange({ [name]: trg.checked ? CANVAS_W : null })
        break
      }
      case 'invertYwithHeight': {
        this.emitChange({ [name]: trg.checked ? CANVAS_H : null })
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

        <div className="field checkbox is-horizontal">
          <label className="label">invert_x_with_width ({CANVAS_W}):</label>
          <div className="control">
            <label className="checkbox">
              <input
                name="invertXwithWidth"
                type="checkbox"
                checked={isNum(settings.invertXwithWidth)}
                onChange={this.handleChange}
              />
            </label>
          </div>
        </div>

        <div className="field checkbox is-horizontal">
          <label className="label">invert_y_with_height ({CANVAS_H}):</label>
          <div className="control">
            <label className="checkbox">
              <input
                name="invertYwithHeight"
                type="checkbox"
                checked={isNum(settings.invertYwithHeight)}
                onChange={this.handleChange}
              />
            </label>
          </div>
        </div>

      </div>

    )
  }

}
