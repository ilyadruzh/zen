import React, { Component } from 'react';

const RoleContext = React.createContext();

class RoleProvider extends Component {
    state = {
        isAuth: true
    }

    render() {
        return (<RoleContext.Provider value={{ isAuth: this.state.isAuth }} >
            {this.props.children} </RoleContext.Provider>)
    }
}

const RoleConsumer = RoleContext.Consumer

export {
    RoleProvider,
    RoleConsumer
}
