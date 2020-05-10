import React from 'react';
import { render } from 'react-dom';
import { ThemeProvider } from '@material-ui/core';
import { Router } from 'react-router-dom';
import { history } from './utils/networkUtilities/index';
import theme from './theme';
import App from './App';

render(
  // <Provider>
  <Router history={history}>
    <ThemeProvider theme={theme}>
      <App />
    </ThemeProvider>
  </Router>,
  // </Provider>,
  document.getElementById('root')
);
