// @flow
import React, { Component } from 'react';
import { Link } from 'react-router-dom';

export default class SocialNetworks extends Component {

    render() {
        return (
            <div className="social-login-content">
                <div className="social-button">
                    <button className="au-btn au-btn--block au-btn--blue m-b-20">sign in with facebook</button>
                    <button className="au-btn au-btn--block au-btn--blue2">sign in with twitter</button>
                </div>
            </div>
        );
    }
}
