// @flow
import React, { Component, Fragment } from 'react';
import { Link } from 'react-router-dom';
import styles from './Home.css';

import TopMenu from './Menu/TopMenu/TopMenu';
import LeftMenu from './Menu/LeftMenu';
import MainFrame from './Main/MainFrame';
import RightMenu from './Menu/RightMenu';
import BottomMenu from './Menu/BottomMenu';

type Props = {};

export default class Home extends Component<Props> {
    props: Props;

    render() {
        return (
            <div className="animsition">
                <div className="page-wrapper">
                    <TopMenu />
                    <LeftMenu />
                    <MainFrame />
                    <footer><BottomMenu /></footer>
                </div>
            </div>
        );
    }
}
