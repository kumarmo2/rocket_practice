const emailRegex = /^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,3})+$/;
const peopleNameRegex = /^[a-zA-Z ]+$/;

export const isValidEmail = (email) => {
  if (!email) {
    return false;
  }
  return emailRegex.test(email);
};

export const isValidName = (name) => {
  if (!name) {
    return false;
  }
  if (!name.trimLeft()) {
    return false;
  }
  return peopleNameRegex.test(name);
};
