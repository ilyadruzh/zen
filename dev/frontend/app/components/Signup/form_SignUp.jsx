// @flow
import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import FieldsSignup from './FiledsSignUp';
import AggrementSignup from './checkBox_Aggrement';
import ButtonSignUp from './button_SignUp';
import ButtonSocialNetworks from './button_SocialNetworks'
import ButtonSignIn from './button_SignIn';

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
