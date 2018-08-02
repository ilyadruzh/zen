// @flow
import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import styles from './Signup.css';
import LogoSignup from './logo_SignUp';
import FormSignup from './form_SignUp'

export default class Signup extends Component {

    render() {
        return (
            <div className="page-wrapper">
                <div className="page-content--bge5">
                    <div className="container">
                        <div className="login-wrap">
                            <div className="login-content">
                                <LogoSignup />
                                <FormSignup />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        );
    }
}
