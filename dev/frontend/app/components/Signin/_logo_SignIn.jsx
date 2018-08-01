// @flow
import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import styles from './LogoSignIn.css';

export default class LogoSignIn extends Component {

    render() {
        return (
            <div className="login-logo">
            <a href="#">
                <img src="./assets/images/icon/logo.png" />
            </a>
        </div>
        );
    }
}
