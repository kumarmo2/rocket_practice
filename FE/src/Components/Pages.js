import React, { Component } from 'react';
import { Switch, Route } from 'react-router-dom';
import Home from './Home/Home';
import SignInSignup from '../Components/SignUpSignIn/components/SignUpSignIn';
import { signInType } from '../Components/SignUpSignIn/constants/';

export default class Pages extends Component {
  render() {
    return (
      <Switch>
        <Route path="/signup" exact component={SignInSignup} />
        <Route
          path="/signin"
          exact
          render={props => {
            return <SignInSignup {...props} variant={signInType} />;
          }}
        />
        <Route path="/" exact component={Home} />
      </Switch>
    );
  }
}
