// компонент отображающий список постов человека и его друзей

import React, { Component } from 'react'

import './SearchBar.css'

export default class SearchBar extends Component {

    constructor(props) {
        super(props)
    }

    render() {
        return (
            <form className="form-header" action="" method="POST">
            <input className="au-input au-input--xl" type="text" name="search" placeholder="Search for datas &amp; reports..." />
            <button className="au-btn--submit" type="submit">
                <i className="zmdi zmdi-search"></i>
            </button>
        </form>
        )
    }

}