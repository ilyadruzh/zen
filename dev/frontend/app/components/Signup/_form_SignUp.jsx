// @flow
import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import styles from './FormSignup.css';
import FieldsSignup from './FiledsSignUp';
import AggrementSignup from './_checkBox_Aggrement';
import ButtonSignUp from './_button_SignUp';
import ButtonSocialNetworks from './_button_SocialNetworks'
import ButtonSignIn from './_button_SignIn';

export default class FormSignup extends Component {

    render() {
        return (
            <div className="login-form">
            
                <form action="" method="post">

                    <FieldsSignup />
                    <AggrementSignup />
                    <ButtonSignUp />
                    <ButtonSocialNetworks />

                </form>

                <ButtonSignIn />
            </div>

        );
    }
}
