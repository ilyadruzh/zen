// компонент отображающий список постов человека и его друзей

import React, { Component } from 'react'

import SearchBar from './SearchBar'
import NotifyBar from './NotifyBar'
import ProfileInfo from './ProfileInfo'

import './TopMenu.css'

export default class TopMenu extends Component {

    constructor(props) {
        super(props)
    }

    render() {
        return (
            <header className="header-desktop">
                <div className="section__content section__content--p30">
                    <div className="container-fluid">
                        <div className="header-wrap">
                            <SearchBar />
                            <div className="header-button">
                                <NotifyBar />
                                <ProfileInfo />
                            </div>
                        </div>
                    </div>
                </div>
            </header>
        )
    }
}