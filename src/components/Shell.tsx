import React, { useState } from "react";
import { AppShell, Navbar, Header, Title, Text, Button } from "@mantine/core";


function AppNav() {
  return (
    <Navbar 
      width={{ base: 300 }} 
      // height={500} 
      p="xs"
    >
      {/* Navbar content */}
    </Navbar>
  );
}


function AppHeader() {
  return (
    <Header 
      height={60} 
      p="xs"
    >
      <Title order={3}>
        SQL-NB
      </Title>
      {/* Header content */}
    </Header>
  );
}


export interface IShellProps {
  children: React.ReactNode;
}

export default function Shell({children}: IShellProps) {
  return (
    <AppShell
      padding="md"
      navbar={<AppNav />}
      header={<AppHeader />}
      styles={(theme) => ({
        main: { backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[8] : theme.colors.gray[0] },
      })}
    >
      { children }
    </AppShell>
  );
}
