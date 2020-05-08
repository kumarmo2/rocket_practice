export const createUser = ({ email, fullName, password }) => {
  if (!email || !fullName || !password) {
    return Promise.reject('Enter All details');
  }
  return fetch('/api/users/', {
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
  return fetch('/api/signin', {
    method: 'POST',
    body: JSON.stringify({
      email,
      password,
    }),
  });
};

export const signOut = () => {
  return fetch('/api/signout', {
    method: 'POSt',
  });
};
