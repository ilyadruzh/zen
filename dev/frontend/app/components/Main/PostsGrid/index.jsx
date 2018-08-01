// компонент отображающий список постов человека и его друзей

import React, { Component } from 'react'
import PostEntity from '../PostEntity'
import './PostsGrid.css'

export default class PostsGrid extends Component {


    constructor(props){
        super(props)
    }

    render() {
        return (
            <div className="posts-grid">
                {
                    this.props.posts.map(post => {
                        return <PostEntity key={post.id} color={post.color}> {post.text} </PostEntity>
                    })
                }
            </div>
        )
    }

}