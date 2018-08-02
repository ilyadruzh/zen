// @flow
import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import LogoSignup from './logo_SignUp';
import FormSignup from './form_SignUp'

export default class ButtonSignIn extends Component {

    render() {
        return (
            <div className="register-link">
                <p>
                    Already have account? <a href="#">Sign In</a>
                </p>
            </div>
        );
    }
}
