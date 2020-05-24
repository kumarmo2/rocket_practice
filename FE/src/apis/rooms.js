import { api } from '../utils/networkUtilities/FetchWrapper';

export const create = ({ creatorUserId, roomName }) => {
  if (creatorUserId < 1) {
    return Promise.reject('Invalid user id');
  }
  if (!roomName || !roomName.length) {
    return Promise.reject('Must provide room name');
  }
  return api('/api/rooms', {
    method: 'POST',
    body: JSON.stringify({
      creatorUserId,
      roomName,
    }),
  });
};
