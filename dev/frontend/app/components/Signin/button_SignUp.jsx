// @flow
import React, { Component } from 'react';
import { Link } from 'react-router-dom';

export default class ButtonSignUp extends Component {

    render() {
        return (
            <div className="register-link">
                <p>
                    Don't you have account? <Link to="signup">Sign Up Here</Link>
                </p>
            </div>
        );
    }
}
