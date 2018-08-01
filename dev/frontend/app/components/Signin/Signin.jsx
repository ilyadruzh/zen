// @flow
import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import styles from './Signin.css';
import SocialNetworks from './SocialNetworks'
import FieldsForAuth from './FieldsForAuth'
import SignInButton from './SignInButton'
import CheckBoxRemeberMe from './_checkBoxRemeberMe'
import CheckBoxForgottenPassword from './_checkBoxForgottenPassword'
import ButtonSignUp from './_button_SignUp'
import LogoSignIn from './_logo_SignIn'
export default class Signin extends Component {

    render() {
        return (
            <div className="page-wrapper">
                <div className="page-content--bge5">
                    <div className="container">
                        <div className="login-wrap">
                            <div className="login-content">

                                <LogoSignIn />

                                <div className="login-form">

                                    <form action="" method="post">

                                        <FieldsForAuth />
                                        <div className="login-checkbox">
                                            <CheckBoxRemeberMe />
                                            <CheckBoxForgottenPassword />
                                        </div>
                                        <SignInButton />
                                        <SocialNetworks />

                                    </form>

                                    <ButtonSignUp />

                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        );
    }
}
