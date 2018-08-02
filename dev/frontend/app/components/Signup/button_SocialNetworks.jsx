// @flow
import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import styles from './Signup.css';
import LogoSignup from './logo_SignUp';
import FormSignup from './form_SignUp'

export default class ButtonSocialNetworks extends Component {

    render() {
        return (
            <div className="social-login-content">
                <div className="social-button">
                    <button className="au-btn au-btn--block au-btn--blue m-b-20">register with facebook</button>
                    <button className="au-btn au-btn--block au-btn--blue2">register with twitter</button>
                </div>
            </div>
        );
    }
}
