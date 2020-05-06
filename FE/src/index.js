import React from 'react';
import { render } from 'react-dom';
import App from './App';
import { ThemeProvider, RootRef } from '@material-ui/core';
import { Provider } from 'react-redux';
import theme from "./theme";
import { BrowserRouter as Router } from 'react-router-dom';


render(
    <Router>
        <ThemeProvider theme={theme}>
            <App />
        </ThemeProvider>
    </Router>
    , document.getElementById("root"));
