import { SET_USER_INFO } from '../actions/user';
export const setUserInfo = ({ name, email, userId }) => {
  return {
    type: SET_USER_INFO,
    name,
    email,
    userId,
  };
};
