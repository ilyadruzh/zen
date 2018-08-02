// @flow
import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import styles from './Signup.css';
import LogoSignup from './logo_SignUp';
import FormSignup from './form_SignUp'

export default class AggrementSignup extends Component {

    render() {
        return (
            <div className="login-checkbox">
            <label>
                <input type="checkbox" name="aggree" />Agree the terms and policy
            </label>
        </div>
        );
    }
}
