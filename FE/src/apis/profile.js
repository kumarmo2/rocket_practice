import { api } from '../utils/networkUtilities/FetchWrapper';

export const get = () => {
  return api('/me/profile');
};
