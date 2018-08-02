// @flow
import React, { Component } from 'react';
import { Link } from 'react-router-dom';

export default class CheckBoxRemeberMe extends Component {

    render() {
        return (
            <label>
            <input type="checkbox" name="remember" />Remember Me
        </label>       
         );
    }
}
