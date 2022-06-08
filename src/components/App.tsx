import React, { useState } from 'react';
import {
  Button,
  Navbar,
  Alignment,
} from "@blueprintjs/core";
import Editor from 'react-simple-code-editor';


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
  const [code, setCode] = React.useState(
    `function add(a, b) {\n  return a + b;\n}`
  );
  return (
    <Editor
      value={code}
      onValueChange={code => setCode(code)}
      highlight={code => code}
      // highlight={code => Prism.highlight(code, Prism.languages.js)}
      padding={10}
      style={{
        fontFamily: '"Fira code", "Fira Mono", monospace',
        fontSize: 12,
      }}
    />
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
