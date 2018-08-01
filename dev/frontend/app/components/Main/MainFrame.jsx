// основной фрейм в котором отображается информация

import React, { Component } from 'react'

import NewPost from './NewPost'
import PostsGrid from './PostsGrid'

import NewPostMenu from './NewPost'

import './MainFrame.css'

export default class MainFrame extends Component {

    render() {
        return (
            <div className="main-content">
                <div className="section__content section__content--p30">
                    <div className="container-fluid">

                        <NewPostMenu />

                        {/* <PostsGrid /> */}

                    </div>
                </div>
            </div>
        )
    }
}