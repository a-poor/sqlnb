import React from "react";

import Notebook from "./Notebook";
import EmptyNotebook from "./EmptyNotebook";
import NbHeader from "./NbHeader";


export interface INbContainerProps {
  notebooks: any[];
}

export default function NbContainer({notebooks}: INbContainerProps) {
  if (notebooks.length === 0) {
    return <EmptyNotebook onNew={() => alert("TODO - Add a 'new-notebook' button...")} />;
  }
  return (
    <>
    </>
  );
}
