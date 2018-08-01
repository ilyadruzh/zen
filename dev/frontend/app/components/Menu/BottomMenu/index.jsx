// компонент отображающий список постов человека и его друзей

import React, { Component } from 'react'

import './BottomMenu.css'

export default class BottomMenu extends Component {

    constructor(props) {
        super(props)
    }

    render() {
        return (
            <div className="row">
            <div className="col-md-12">
                <div className="copyright">
                    <p>Copyright © 2018 Colorlib. All rights reserved. Template by <a href="https://colorlib.com">Colorlib</a>.</p>
                </div>
            </div>
        </div>
        )
    }

}