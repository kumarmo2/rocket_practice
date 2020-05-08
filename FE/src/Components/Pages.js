import React, { Component } from 'react';
import { Switch, Route } from 'react-router-dom';
import SignUp from './SignUp';
import Home from './Home/Home';
import { ChatListing } from './ChatListing';

export default class Pages extends Component {
  render() {
    return (
      <Switch>
        <Route path="/signup" exact component={SignUp} />
        <Route path="/" exact component={Home} />
        <Route path="/chat" exact component={ChatListing} />
      </Switch>
    );
  }
}
