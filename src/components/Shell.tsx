import React, { useState } from "react";
import { AppShell, Navbar, Header, Title, Text, Button, Box, NavLink, Aside, MediaQuery, ActionIcon, Stack, Divider, Breadcrumbs, Anchor } from "@mantine/core";
import { IconFolders, IconFolder, IconFile, IconNotebook, IconDatabase, IconSettings } from '@tabler/icons';


const workingDirectoryPath = [
  "Users",
  "austinpoor",
  "Desktop",
  "code",
  "etc",
  "sqlnb"
];

const dummyDirContents = [
  {name: ".git", isDir: true},
  {name: "data", isDir: true},
  {name: "static", isDir: true},
  {name: "tmp", isDir: true},
  {name: ".gitignore", isDir: false},
  {name: "config.json", isDir: false},
  {name: "eda.sql.nb", isDir: false},
  {name: "info.txt", isDir: false},
  {name: "my-notebook.sql.nb", isDir: false},
  {name: "notebook-1.sql.nb", isDir: false},
];

enum NavSection {
  Directory = 0,
  Notebooks,
  Databases,
  Settings,
}

interface INavDetailsProps {
  section?: NavSection;
  dirPath: string[];
  dirContents: {
    name: string;
    isDir: boolean;
    modified?: Date;
  }[];
}

function NavDetails({section, dirPath, dirContents}: INavDetailsProps) {
  return (
    <>
      <Divider orientation="vertical" mx="xs" />
      <div
        style={{
          height: "100%",
          flexGrow: 4,
        }}
      >
        <Text weight={500}>
          {section === NavSection.Directory && "Directory"}
          {section === NavSection.Notebooks && "Notebooks"}
          {section === NavSection.Databases && "Databases"}
          {section === NavSection.Settings && "Settings"}
        </Text>
        <Divider my="xs" size={1} color="rgba(0,0,0,0.1)" />

        <Breadcrumbs>
          <Anchor component="button" type="button">
            ..
          </Anchor>
          <Anchor component="button" type="button">
            {dirPath[dirPath.length - 1]}
          </Anchor>
        </Breadcrumbs>
        <Divider my="xs" size={1} color="rgba(0,0,0,0.1)" />

        <Box>
          {dirContents.map((d, i) => (
            <NavLink 
              key={i}
              label={d.name}
              icon={d.isDir ? <IconFolder /> : <IconFile />}
              description={d.modified ? d.modified.toLocaleString() : undefined}
            />
          ))}
        </Box>
      </div>
    </>
  );
}

function AppNav() {
  const [activeSection, setActiveSection] = useState<NavSection | undefined>(NavSection.Directory);
  return (
    <Navbar 
      hiddenBreakpoint="sm"
      width={{ 
        base: activeSection !== undefined ? 300 : 65,
      }}
      p="xs"
    >
      <div
        style={{
          display: "flex",
          height: "100%",
        }}
      >
        <div
          style={{
            height: "100%",
          }}
        >
          <Stack 
            spacing={5}
            align="center"
          >
            <ActionIcon 
              onClick={() => activeSection === NavSection.Directory ? setActiveSection(undefined) : setActiveSection(NavSection.Directory)}
              color={activeSection === NavSection.Directory ? "blue" : undefined}
              variant={activeSection === NavSection.Directory ? "light" : "subtle"}
              size="xl" 
            >
              <IconFolders />
            </ActionIcon>
            <ActionIcon 
              onClick={() => activeSection === NavSection.Notebooks ? setActiveSection(undefined) : setActiveSection(NavSection.Notebooks)}
              color={activeSection === NavSection.Notebooks ? "blue" : undefined}
              variant={activeSection === NavSection.Notebooks ? "light" : "subtle"}
              size="xl" 
            >
              <IconNotebook />
            </ActionIcon>
            <ActionIcon 
              onClick={() => activeSection === NavSection.Databases ? setActiveSection(undefined) : setActiveSection(NavSection.Databases)}
              color={activeSection === NavSection.Databases ? "blue" : undefined}
              variant={activeSection === NavSection.Databases ? "light" : "subtle"}
              size="xl" 
            >
              <IconDatabase />
            </ActionIcon>
            <ActionIcon 
              onClick={() => activeSection === NavSection.Settings ? setActiveSection(undefined) : setActiveSection(NavSection.Settings)}
              color={activeSection === NavSection.Settings ? "blue" : undefined}
              variant={activeSection === NavSection.Settings ? "light" : "subtle"}
              size="xl" 
            >
              <IconSettings />
            </ActionIcon>
          </Stack>
        </div>
        
        {activeSection !== undefined && (
          <NavDetails 
            section={activeSection}
            dirPath={workingDirectoryPath}
            dirContents={dummyDirContents}
          />
        )}
      </div>
    </Navbar>
  );
}


function AppAside() {
  return (
    <MediaQuery smallerThan="sm" styles={{ display: 'none' }}>
      <Aside p="md" hiddenBreakpoint="sm" width={{ sm: 200, lg: 300 }}>
        <Text>AppAside</Text>
      </Aside>
    </MediaQuery>
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
      aside={<AppAside />}
      styles={(theme) => ({
        main: { backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[8] : theme.colors.gray[0] },
      })}
    >
      { children }
    </AppShell>
  );
}
