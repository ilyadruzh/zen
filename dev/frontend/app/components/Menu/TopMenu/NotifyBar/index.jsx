// компонент отображающий список постов человека и его друзей

import React, { Component } from 'react'

import './NotifyBar.css'

export default class NotifyBar extends Component {

    constructor(props) {
        super(props)
    }

    render() {
        return (
<div className="noti-wrap">
                                    <div className="noti__item js-item-menu">
                                        <i className="zmdi zmdi-comment-more"></i>
                                        <span className="quantity">1</span>
                                        <div className="mess-dropdown js-dropdown">
                                            <div className="mess__title">
                                                <p>You have 2 news message</p>
                                            </div>
                                            <div className="mess__item">
                                                <div className="image img-cir img-40">
                                                    {/* <img src="icon/avatar-06.jpg" alt="Michelle Moreno" /> */}
                                                </div>
                                                <div className="content">
                                                    <h6>Michelle Moreno</h6>
                                                    <p>Have sent a photo</p>
                                                    <span className="time">3 min ago</span>
                                                </div>
                                            </div>
                                            <div className="mess__item">
                                                <div className="image img-cir img-40">
                                                    {/* <img src="icon/avatar-04.jpg" alt="Diane Myers" /> */}
                                                </div>
                                                <div className="content">
                                                    <h6>Diane Myers</h6>
                                                    <p>You are now connected on message</p>
                                                    <span className="time">Yesterday</span>
                                                </div>
                                            </div>
                                            <div className="mess__footer">
                                                <a href="#">View all messages</a>
                                            </div>
                                        </div>
                                    </div>
                                    <div className="noti__item js-item-menu">
                                        <i className="zmdi zmdi-email"></i>
                                        <span className="quantity">1</span>
                                        <div className="email-dropdown js-dropdown">
                                            <div className="email__title">
                                                <p>You have 3 New Emails</p>
                                            </div>
                                            <div className="email__item">
                                                <div className="image img-cir img-40">
                                                    {/* <img src="icon/avatar-06.jpg" alt="Cynthia Harvey" /> */}
                                                </div>
                                                <div className="content">
                                                    <p>Meeting about new dashboard...</p>
                                                    <span>Cynthia Harvey, 3 min ago</span>
                                                </div>
                                            </div>
                                            <div className="email__item">
                                                <div className="image img-cir img-40">
                                                    {/* <img src="icon/avatar-05.jpg" alt="Cynthia Harvey" /> */}
                                                </div>
                                                <div className="content">
                                                    <p>Meeting about new dashboard...</p>
                                                    <span>Cynthia Harvey, Yesterday</span>
                                                </div>
                                            </div>
                                            <div className="email__item">
                                                <div className="image img-cir img-40">
                                                    {/* <img src="icon/avatar-04.jpg" alt="Cynthia Harvey" /> */}
                                                </div>
                                                <div className="content">
                                                    <p>Meeting about new dashboard...</p>
                                                    <span>Cynthia Harvey, April 12,,2018</span>
                                                </div>
                                            </div>
                                            <div className="email__footer">
                                                <a href="#">See all emails</a>
                                            </div>
                                        </div>
                                    </div>
                                    <div className="noti__item js-item-menu">
                                        <i className="zmdi zmdi-notifications"></i>
                                        <span className="quantity">3</span>
                                        <div className="notifi-dropdown js-dropdown">
                                            <div className="notifi__title">
                                                <p>You have 3 Notifications</p>
                                            </div>
                                            <div className="notifi__item">
                                                <div className="bg-c1 img-cir img-40">
                                                    <i className="zmdi zmdi-email-open"></i>
                                                </div>
                                                <div className="content">
                                                    <p>You got a email notification</p>
                                                    <span className="date">April 12, 2018 06:50</span>
                                                </div>
                                            </div>
                                            <div className="notifi__item">
                                                <div className="bg-c2 img-cir img-40">
                                                    <i className="zmdi zmdi-account-box"></i>
                                                </div>
                                                <div className="content">
                                                    <p>Your account has been blocked</p>
                                                    <span className="date">April 12, 2018 06:50</span>
                                                </div>
                                            </div>
                                            <div className="notifi__item">
                                                <div className="bg-c3 img-cir img-40">
                                                    <i className="zmdi zmdi-file-text"></i>
                                                </div>
                                                <div className="content">
                                                    <p>You got a new file</p>
                                                    <span className="date">April 12, 2018 06:50</span>
                                                </div>
                                            </div>
                                            <div className="notifi__footer">
                                                <a href="#">All notifications</a>
                                            </div>
                                        </div>
                                    </div>
                                </div>
        )
    }

}