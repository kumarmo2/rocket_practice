import React, { PureComponent } from 'react';
import { Button } from '@material-ui/core';
import PropTypes from 'prop-types';
import { signOut } from '../SignUpSignIn/apis/';

const propTypes = {
  history: PropTypes.object.isRequired,
};
export default class Home extends PureComponent {
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

Home.propTypes = propTypes;
