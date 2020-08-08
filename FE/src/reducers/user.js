import { SET_USER_INFO } from '../actions/user';

const initialState = {
  userId: 0,
  name: '',
  email: '',
};

export const user = (state = initialState, action) => {
  switch (action.type) {
    case SET_USER_INFO: {
      const { userId, name, email } = action;
      return {
        ...state,
        userId,
        name,
        email,
      };
    }
    default: {
      return state;
    }
  }
};
