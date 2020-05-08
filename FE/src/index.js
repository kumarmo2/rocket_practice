import React from 'react';
import { render } from 'react-dom';
import { ThemeProvider } from '@material-ui/core';
import { BrowserRouter as Router } from 'react-router-dom';
import theme from './theme';
import App from './App';

render(
  // <Provider>
  <Router>
    <ThemeProvider theme={theme}>
      <App />
    </ThemeProvider>
  </Router>,
  // </Provider>,
  document.getElementById('root')
);
