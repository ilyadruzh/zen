// @flow
import React, { Component } from 'react';
import { Link } from 'react-router-dom';

export default class FieldsForAuth extends Component {

    render() {
        return (
            <React.Fragment>
            <div className="form-group">
                                            <label>Email Address</label>
                                            <input className="au-input au-input--full" type="email" name="email" placeholder="Email" />
                                        </div>
                                        <div className="form-group">
                                            <label>Password</label>
                                            <input className="au-input au-input--full" type="password" name="password" placeholder="Password" />
                                        </div>
                                        </React.Fragment>
        );
    }
}
