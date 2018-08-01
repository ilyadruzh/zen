// @flow
import React, { Component } from 'react';
import Signin from '../components/Signin/Signin';
import * as SigninActions from '../actions/signin';

type Props = {};

export default class SigninPage extends Component<Props> {
  props: Props;

  render() {
    return <Signin />;
  }
}
