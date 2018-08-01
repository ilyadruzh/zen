// @flow
import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import styles from './Signup.css';

export default class Signup extends Component {

    render() {
        return (
            <div className="page-wrapper">
                <div className="page-content--bge5">
                    <div className="container">
                        <div className="login-wrap">
                            <div className="login-content">
                                <div className="login-logo">
                                    <a href="#">
                                        <img src="./assets/images/icon/logo.png" />
                                    </a>
                                </div>
                                <div className="login-form">
                                    <form action="" method="post">
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
                                        <div className="login-checkbox">
                                            <label>
                                                <input type="checkbox" name="aggree" />Agree the terms and policy
                                        </label>
                                        </div>
                                        <button className="au-btn au-btn--block au-btn--green m-b-20" type="submit">register</button>
                                        <div className="social-login-content">
                                            <div className="social-button">
                                                <button className="au-btn au-btn--block au-btn--blue m-b-20">register with facebook</button>
                                                <button className="au-btn au-btn--block au-btn--blue2">register with twitter</button>
                                            </div>
                                        </div>
                                    </form>
                                    <div className="register-link">
                                        <p>
                                            Already have account?
                                        <a href="#">Sign In</a>
                                        </p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

            </div>
        );
    }
}
