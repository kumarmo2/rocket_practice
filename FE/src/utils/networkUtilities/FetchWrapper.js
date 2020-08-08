import { history as historyObj } from './index';

const commonHeaders = {
  'Content-Type': 'application/json',
};

export const api = (url, options = {}) => {
  if (!url) {
    return Promise.reject('url must be provided');
  }
  let { headers } = options;
  let newheaders =
    (headers && { ...commonHeaders, ...headers }) || commonHeaders;
  options.headers = newheaders;
  return fetch(url, options).then(response => {
    if (response.status === 401) {
      historyObj.replace('/signin', '');
      // TODO: if unauthorized, we just want to redirect to signin, no further logic ideally should be executed. confirm that.
    }
    return response;
  });
};
