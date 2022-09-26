import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import { MantineProvider, Text } from '@mantine/core';

import Shell from "./components/Shell";
import NbContainer from "./components/NbContainer";


function App() {
  return (
    <MantineProvider withGlobalStyles withNormalizeCSS>
      <Shell>
        <NbContainer notebooks={[]}/>
      </Shell>
    </MantineProvider>
  );
}
export default App;
