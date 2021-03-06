import { history as historyObj } from './index';

export const api = (url, options = {}) => {
  if (!url) {
    return Promise.reject('url must be provided');
  }
  return fetch(url, options).then(response => {
    if (response.status === 401) {
      historyObj.replace('/signin', '');
      // TODO: if unauthorized, we just want to redirect to signin, no further logic ideally should be executed. confirm that.
    }
    return response;
  });
};
