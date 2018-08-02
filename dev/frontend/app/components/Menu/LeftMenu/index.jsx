// компонент отображающий список постов человека и его друзей

import React, { Component } from 'react'

import './LeftMenu.css'
import { MenuItem } from 'electron';
import MenuList from './MenuList';
import MenuListItem from './MenuListItem';

export default class LeftMenu extends Component {

    constructor(props) {
        super(props)
    }

    render() {
        return (
            <React.Fragment>
                <aside className="menu-sidebar d-none d-lg-block">
                    
                    <div className="logo">
                        <a href="#">
                            {/* <img src="../../assets/images/icon/logo.png" alt="Cool Admin" /> */}
                        </a>
                    </div>

                    <div className="menu-sidebar__content js-scrollbar1">
                        <nav className="navbar-sidebar">
                            <ul className="list-unstyled navbar__list">
                                <li className="active has-sub">
                                    <a className="js-arrow" href="#">
                                        <i className="fas fa-tachometer-alt"></i>Home</a>
                                    <ul className="list-unstyled navbar__sub-list js-sub-list">
                                        <li>
                                            <a href="index.html">Dashboard 1</a>
                                        </li>
                                        <li>
                                            <a href="index2.html">Dashboard 2</a>
                                        </li>
                                        <li>
                                            <a href="index3.html">Dashboard 3</a>
                                        </li>
                                        <li>
                                            <a href="index4.html">Dashboard 4</a>
                                        </li>
                                    </ul>
                                </li>
                                <li>
                                    <a href="chart.html">
                                        <i className="fas fa-chart-bar"></i>News</a>
                                </li>
                                <li>
                                    <a href="table.html">
                                        <i className="fas fa-table"></i>Exchange</a>
                                </li>
                                <li>
                                    <a href="form.html">
                                        <i className="far fa-check-square"></i>Calls</a>
                                </li>
                                <li>
                                    <a href="form.html">
                                        <i className="far fa-check-square"></i>Market</a>
                                </li>
                                <li>
                                    <a href="form.html">
                                        <i className="far fa-check-square"></i>Lightning Network</a>
                                </li>
                                <li>
                                    <a href="#">
                                        <i className="fas fa-calendar-alt"></i>Messenger</a>
                                </li>
                                <li>
                                    <a href="map.html">
                                        <i className="fas fa-map-marker-alt"></i>Ratings</a>
                                </li>
                                <li className="has-sub">
                                    <a className="js-arrow" href="#">
                                        <i className="fas fa-copy"></i>Token Sale</a>
                                    <ul className="list-unstyled navbar__sub-list js-sub-list">
                                        <li>
                                            <a href="login.html">Login</a>
                                        </li>
                                        <li>
                                            <a href="register.html">Register</a>
                                        </li>
                                        <li>
                                            <a href="forget-pass.html">Forget Password</a>
                                        </li>
                                    </ul>
                                </li>
                                <li className="has-sub">
                                    <a className="js-arrow" href="#">
                                        <i className="fas fa-desktop"></i>Settings</a>
                                    <ul className="list-unstyled navbar__sub-list js-sub-list">
                                        <li>
                                            <a href="button.html">Button</a>
                                        </li>
                                        <li>
                                            <a href="badge.html">Badges</a>
                                        </li>
                                        <li>
                                            <a href="tab.html">Tabs</a>
                                        </li>
                                        <li>
                                            <a href="card.html">Cards</a>
                                        </li>
                                        <li>
                                            <a href="alert.html">Alerts</a>
                                        </li>
                                        <li>
                                            <a href="progress-bar.html">Progress Bars</a>
                                        </li>
                                        <li>
                                            <a href="modal.html">Modals</a>
                                        </li>
                                        <li>
                                            <a href="switch.html">Switchs</a>
                                        </li>
                                        <li>
                                            <a href="grid.html">Grids</a>
                                        </li>
                                        <li>
                                            <a href="fontawesome.html">Fontawesome Icon</a>
                                        </li>
                                        <li>
                                            <a href="typo.html">Typography</a>
                                        </li>
                                    </ul>
                                </li>
                            </ul>
                        </nav>
                    </div>
                </aside>
            </React.Fragment>
        )
    }
}