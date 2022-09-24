import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import { MantineProvider, Text } from '@mantine/core';

import Shell from "./components/Shell";


function App() {
  return (
    <MantineProvider withGlobalStyles withNormalizeCSS>
      <Shell>
        <Text>Welcome to Mantine!</Text>
      </Shell>
    </MantineProvider>
  );
}
export default App;
