import React from 'react';
import { render } from 'react-dom';
import { ThemeProvider, RootRef } from '@material-ui/core';
import { Provider } from 'react-redux';
import { BrowserRouter as Router } from 'react-router-dom';
import theme from './theme';
import App from './App';

render(
  <Router>
    <ThemeProvider theme={theme}>
      <App />
    </ThemeProvider>
  </Router>,
  document.getElementById('root'),
);
