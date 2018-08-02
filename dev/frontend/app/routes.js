/* eslint flowtype-errors/show-errors: 0 */
import React from 'react';
import { Switch, Route } from 'react-router';
import { AuthProvider, AuthConsumer } from './context/AuthContext';
import App from './containers/App';
import HomePage from './containers/HomePage';
// import VideoAudioCalls from './containers/VideoAudioCalls';
import SignIn from './containers/Signin';
import SignUp from './containers/Signup';


export default () => (
  <AuthProvider>

    <App>
      <Switch>

        <AuthConsumer>

          {({ isAuth }) => (
            <React.Fragment>
              {isAuth ? (
                <Route path="/" component={HomePage} />
              ) : (
                  <React.Fragment>
                    <Route path="/" component={HomePage} />
                    {/* <Route path="signin" component={SignIn} /> */}
                  </React.Fragment>
                )
              }
            </React.Fragment>
          )}

        </AuthConsumer>
      </Switch>
    </App>

  </AuthProvider>

);
