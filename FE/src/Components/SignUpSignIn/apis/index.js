import { api } from '../../../utils/networkUtilities/FetchWrapper';
export const createUser = ({ email, fullName, password }) => {
  if (!email || !fullName || !password) {
    return Promise.reject('Enter All details');
  }
  // return fetch('/api/users/', {
  return api('/api/users/', {
    method: 'POST',
    body: JSON.stringify({
      email,
      name: fullName,
      password,
    }),
  });
};

export const signIn = (email, password) => {
  if (!email || !password) {
    return Promise.reject();
  }
  return api('/api/signin', {
    method: 'POST',
    body: JSON.stringify({
      email,
      password,
    }),
  });
};

export const signOut = () => {
  return api('/api/signout', {
    method: 'POSt',
  });
};
