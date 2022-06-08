import * as React from 'react';
import * as ReactDOM from 'react-dom';

import "normalize.css";
import "@blueprintjs/core/lib/css/blueprint.css";
// import "@blueprintjs/icons/lib/css/blueprint-icons.css";

import App from './components/App';


function render() {
  ReactDOM.render(
    <App />, 
    document.body,
  );
}

render();
