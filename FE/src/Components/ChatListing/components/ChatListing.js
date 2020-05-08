import React, { PureComponent } from 'react';
import { Button } from '@material-ui/core';
import { signOut } from '../../SignUp/apis';

export default class ChatListing extends PureComponent {
  handleSignOutClient = () => {
    signOut().then(response => {
      if (response.ok) {
        const { history } = this.props;
        history && history.replace('/signup');
      }
    });
  };

  render() {
    return (
      <>
        <Button
          onClick={this.handleSignOutClient}
          color="primary"
          variant="text"
        >
          Sign Out
        </Button>
        <h1>ChatListing page.</h1>
      </>
    );
  }
}
