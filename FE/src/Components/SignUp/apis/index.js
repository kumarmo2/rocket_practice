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

export const signOut = () => {
  return fetch('/signout', {
    method: 'POSt',
  });
};
