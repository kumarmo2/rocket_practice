import React, { PureComponent } from 'react';
import { Container, Box, TextField, Button, withStyles } from '@material-ui/core';
import { isValidEmail, isValidName } from "../../utils/inputValidations";
import { createUser } from "./apis";

const style = {
    root: {
        marginTop: "16px",
    }
}
const StyledPasswordInput = withStyles(style)(TextField);

export default class SignUp extends PureComponent {
    constructor(props) {
        super(props);
        this.state = {
            email: {
                value: "",
                error: false,
                errorText: "",
            },
            fullName: {
                value: "",
                error: false,
                errorText: "",
            },
            password: {
                value: "",
                error: false,
                errorText: "",
            },
            apiErrorMessage: "",
        }
    }

    handleEmailChange = event => {
        let value = event.currentTarget && event.currentTarget.value || "";
        value = value.trimLeft();
        if (!value) {
            this.setState({
                email: {
                    value: value,
                    error: true,
                    errorText: "Email cannot be empty"
                }
            });
            return;
        }

        const isValid = isValidEmail(value);
        if (!isValid) {
            this.setState({
                email: {
                    value: value,
                    error: true,
                    errorText: "Invalid Email",
                }
            });
            return;
        }

        this.setState({
            email: {
                value: value,
                error: false,
                errorText: ""
            }
        });
    }

    handleNameChange = event => {
        let value = (event.currentTarget && event.currentTarget.value) || "";
        value = value.trimLeft();
        if (value.length < 3 || value.length > 100) {
            this.setState({
                fullName: {
                    value,
                    error: true,
                    errorText: "name should be between 3 and 100 characters",
                }
            });
            return;
        }
        if (!isValidName(value)) {
            this.setState({
                fullName: {
                    value,
                    error: true,
                    errorText: "Invalid name",
                }
            });
            return;
        }
        this.setState({
            fullName: {
                value,
                error: false,
                errorText: "",
            }
        });
    }

    handlePasswordChange = event => {
        let value = (event.currentTarget && event.currentTarget.value) || "";
        value = value.trimLeft();
        if (value.length < 6 || value.length > 50) {
            this.setState({
                password: {
                    value,
                    error: true,
                    errorText: "Password must be between 6 and 50 characters"
                }
            });
            return;
        }
        this.setState({
            password: {
                value,
                error: false,
                errorText: "",
            }
        });
    }

    areAllInputValuesValid = () => {
        const { email, fullName, password } = this.state;
        return email.value
            && !email.error
            && fullName.value
            && !fullName.error
            && password.value
            && !password.error;
    }

    handleSignUpClick = () => {
        const { email, password, fullName } = this.state;
        const emailValue = email.value;
        const passwordValue = password.value;
        const name = fullName.value;

        createUser({
            email: emailValue,
            fullName: name,
            password: passwordValue
        })
            .then(response => {
                if (response.ok) {
                    const { history } = this.props;
                    history && history.replace("/chat");
                }
            })
    }

    render() {
        // TODO: in the backend, start accepting the password as well.
        const { fullName, password } = this.state;
        const isSubmitBtnDisabled = !this.areAllInputValuesValid();
        return (
            <Container maxWidth="sm">
                <TextField
                    autoFocus
                    onChange={this.handleEmailChange}
                    value={this.state.email.value}
                    error={this.state.email.error}
                    helperText={this.state.email.errorText}
                    fullWidth placeholder="yourname@email.com"
                    label="Email"
                    required
                />
                <TextField
                    onChange={this.handleNameChange}
                    error={fullName.error}
                    helperText={fullName.errorText}
                    fullWidth placeholder="Full Name"
                    label="Full Name"
                    required
                />
                <StyledPasswordInput
                    onChange={this.handlePasswordChange}
                    error={password.error}
                    helperText={password.errorText}
                    fullWidth
                    type="password"
                    placeholder="6 or more characters"
                    label="Password"
                    required
                />
                <Box display="flex" justifyContent="center" mt={2}>
                    <Button disabled={isSubmitBtnDisabled} onClick={this.handleSignUpClick} color="primary" variant="contained">Sign Up</Button>
                </Box>
            </Container>
        )
    }
}