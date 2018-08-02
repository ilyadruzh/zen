// @flow
import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import styles from './Signup.css';
import LogoSignup from './logo_SignUp';
import FormSignup from './form_SignUp'

export default class ButtonSignup extends Component {

    render() {
        return (
            <button className="au-btn au-btn--block au-btn--green m-b-20" type="submit">register</button>

        );
    }
}
