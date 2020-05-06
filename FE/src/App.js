import React, { Component } from 'react';
import { ThemeProvider } from '@material-ui/core';
import theme from "./theme";
import Pages from "./Components/Pages";


export default class App extends Component {
    render() {
        return (
            <ThemeProvider theme={theme}>
                <Pages />
            </ThemeProvider>
        )
    }
}