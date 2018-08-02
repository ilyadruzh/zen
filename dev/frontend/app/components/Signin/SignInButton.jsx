// @flow
import React, { Component } from 'react';
import { Link } from 'react-router-dom';

export default class SignInButton extends Component {

    render() {
        return (
            <button className="au-btn au-btn--block au-btn--green m-b-20" type="submit">sign in</button>                                      
        );
    }
}
