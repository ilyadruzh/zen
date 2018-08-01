// @flow
import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import styles from './checkBoxForgottenPassword.css';

// TODO: переименовать компоненты и файл в _button_Forgot...
export default class CheckBoxForgottenPassword extends Component {

    render() {
        return (
            <label>
            <a href="#">Forgotten Password?</a>
        </label>   
         );
    }
}
