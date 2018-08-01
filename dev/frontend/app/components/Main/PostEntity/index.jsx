// компоненты отображающий пост

import React, { Component } from 'react'

import './PostEntity.css'

export default class PostEntity extends Component {
    render() {
        return (
            <div className="post">{this.props.children}</div>
        )
    }

}