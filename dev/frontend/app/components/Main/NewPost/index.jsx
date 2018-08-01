// компоненты для создания поста пользователя

import React, { Component } from 'react'

import './NewPostMenu.css'

export default class NewPostMenu extends Component {

    constructor(props) {
        super(props)
    }

    state = {
        text: ''
    }

    handleTextChange(event) {
        this.setState({ text: event.target.value });
    }

    render() {
        return (
            <div className="row">
                <div className="col-md-12">
                    <div className="overview-wrap">
                        <h2 className="title-1">overview</h2>
                        <button className="au-btn au-btn-icon au-btn--blue">
                            <i className="zmdi zmdi-plus"></i>add item</button>
                    </div>
                </div>
            </div>
        )
    }

}