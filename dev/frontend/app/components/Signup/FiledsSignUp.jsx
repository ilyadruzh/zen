// @flow
import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import styles from './Signup.css';
import LogoSignup from './_logo_SignUp';
import FormSignup from './_form_SignUp'

export default class FieldsSignup extends Component {

    render() {
        return (
            <React.Fragment>
                <div className="form-group">
                    <label>Username</label>
                    <input className="au-input au-input--full" type="text" name="username" placeholder="Username" />
                </div>
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
