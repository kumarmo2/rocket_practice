import React, { PureComponent } from 'react';
import { Button, TextField } from '@material-ui/core';
import PropTypes from 'prop-types';
import { signOut } from '../SignUpSignIn/apis/';
import { get as getProfile } from '../../apis/profile';
import { create as createRoomApi } from '../../apis/rooms';
import { setUserInfo } from '../../actionCreators/user';
import { connect } from 'react-redux';
import { bindActionCreators } from 'redux';
import { Link } from 'react-router-dom';

const propTypes = {
  history: PropTypes.object.isRequired,
  setUserInfo: PropTypes.func.isRequired,
  user: PropTypes.object.isRequired,
};
export class Home extends PureComponent {
  constructor(props) {
    super(props);
    this.state = {
      user: {
        rooms: [],
      },
      newRoomName: '',
    };
  }
  handleSignOutClient = () => {
    signOut().then(response => {
      if (response.ok) {
        const { setUserInfo } = this.props;
        setUserInfo({});
        const { history } = this.props;
        history && history.replace('/signup');
      }
    });
  };

  handleRoomNameChange = event => {
    let value = (event.currentTarget && event.currentTarget.value) || '';
    value = value.trimLeft();
    this.setState({
      newRoomName: value,
    });
  };

  handleCreateNewRoomBtnClick = () => {
    const { user } = this.props;
    if (!user || !user.userId) {
      return;
    }
    const { newRoomName } = this.state;
    if (!newRoomName) {
      return;
    }
    createRoomApi({
      creatorUserId: user.userId,
      roomName: newRoomName,
    })
      .then(response => {
        return response.json();
      })
      .then(json => {
        console.log('json: ', json);
      });
  };

  render() {
    const { user } = this.props;
    if (!user || !user.userId) {
      return null;
    }
    const { name, email } = user;
    const { newRoomName, user: userState } = this.state;
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
        <div>Name: {name}</div>
        <div>Email: {email}</div>
        <TextField
          onChange={this.handleRoomNameChange}
          label="Room Name"
          placeholder="Enter Room Name"
        />
        <Button
          color="secondary"
          variant="outlined"
          disabled={!newRoomName.length}
          onClick={this.handleCreateNewRoomBtnClick}
        >
          Create Room
        </Button>
        {userState.rooms && userState.rooms.length > 0 && (
          <>
            <div>My Rooms</div>
            {userState.rooms.map(room => {
              return (
                <div key={room.id}>
                  <Link to={`/rooms/${room.path}`}> {room.name}</Link>
                </div>
              );
            })}
          </>
        )}
      </>
    );
  }

  componentDidMount() {
    getProfile()
      .then(response => {
        // TODO: after all apis start sending the json only, put this call in the fetch wrapper.
        return response.json();
      })
      .then(response => {
        console.log('json response: ', response);
        if (!response || !response.userId) {
          return;
        }
        const { setUserInfo } = this.props;
        const { rooms, ...otherUserInfo } = response;

        rooms &&
          rooms.length &&
          this.setState({
            user: {
              ...this.state.user,
              rooms,
            },
          });

        setUserInfo && setUserInfo(otherUserInfo);
      });
  }
}

Home.propTypes = propTypes;

const mapStateToProps = state => {
  const { user } = state;
  return {
    user,
  };
};

const mapDispatchToProps = dispatch => {
  return {
    setUserInfo: bindActionCreators(setUserInfo, dispatch),
  };
};

export default connect(mapStateToProps, mapDispatchToProps)(Home);
