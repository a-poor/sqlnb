import React, { useState } from 'react';
import {
  Button,
  Navbar,
  Alignment,
} from "@blueprintjs/core";
import Editor from 'react-simple-code-editor';
import { createReactEditorJS } from 'react-editor-js';

import { EDITOR_JS_TOOLS } from './tools';


export function AppBar() {
  return (
    <div className="app-bar">
      <Navbar>
        <Navbar.Group align={Alignment.LEFT}>
          <Navbar.Heading>Blueprint</Navbar.Heading>
          <Navbar.Divider />
          <Button className="bp4-minimal" icon="home" text="Home" />
          <Button className="bp4-minimal" icon="document" text="Files" />
        </Navbar.Group>
      </Navbar>
    </div>
  );
}

export function AppEditor() {
  const ReactEditorJS = createReactEditorJS();
  // const blocks = null;
  return (
    <>
      <ReactEditorJS
        tools={EDITOR_JS_TOOLS}
        // defaultValue={blocks} 
      />
    </>
  );
}

export default function App() {
  return (
    <div id="app">
      <AppBar />
      <h1>Hello, World!</h1>
      <AppEditor />
    </div>
  );
}
