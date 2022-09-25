import React, { useState } from "react";
import { AppShell, Navbar, Header, Title, Text, Button, Tooltip, NavLink, Aside, MediaQuery, ActionIcon, Stack, Divider, Menu, ScrollArea } from "@mantine/core";
import { IconFolders, IconFolder, IconFile, IconNotebook, IconDatabase, IconSettings, IconDots, IconChevronRight } from '@tabler/icons';


const dummyDirContents: (IFileData | IDirectoryData)[] = [
  {name: "data", type: "directory", contents: [
    {name: "_old", type: "directory", contents: [
      {name: "info.txt", type: "file", size: "3.1K"},
    ]},
    {name: "info.txt", type: "file", size: "3.1K"},
    {name: "data.json", type: "file", size: "368B"},
  ]},
  {name: "static", type: "directory", contents: [
    {name: "logo.jpeg", type: "file", size: "128B"},
    {name: "logo.png", type: "file", size: "128B"},
  ]},
  {name: "tmp", type: "directory", contents: [
    {name: "data-1.json", type: "file", size: "128B"},
    {name: "data-2.json", type: "file", size: "559B"},
    {name: "data-3.json", type: "file", size: "378B"},
  ]},
  {name: ".gitignore", type: "file", size: "378B"},
  {name: "config.json", type: "file", size: "733B"},
  {name: "eda.sql.nb", type: "file", size: "3.1K"},
  {name: "info.txt", type: "file", size: "378B"},
  {name: "my-notebook.sql.nb", type: "file", size: "889B"},
  {name: "notebook-1.sql.nb", type: "file", size: "559B"},
];

enum NavSection {
  Directory = 0,
  Notebooks,
  Databases,
  Settings,
}

interface IFileData {
  type: "file";
  name: string;
  size?: string;
}

function FileNav({name, size}: IFileData) {
  return (
    <NavLink 
      onDoubleClick={() => alert("Double clicked file: " + name)}
      label={name}
      icon={<IconFile size={16} stroke={1.5}/>}
      description={size ? size : undefined}
    />
  );
}

interface IDirectoryData {
  type: "directory";
  name: string;
  contents: (IDirectoryData | IFileData)[];
}

function DirNav({name, contents}: IDirectoryData) {
  return (
    <NavLink 
      label={name}
      icon={<IconFolder size={16} stroke={1.5}/>}
      // description={d.modified ? d.modified.toLocaleString() : undefined} // TODO - Number of files in directory?
      childrenOffset={7}
    >
      {contents.map((d, i) => (
        d.type === "directory" ? <DirNav key={i} {...d} /> : <FileNav key={i} {...d} />
      ))}

    </NavLink>
  );
}

interface INavDetailsProps {
  section?: NavSection;
  dirContents: (IDirectoryData | IFileData)[];
}

function NavDetails({section, dirContents}: INavDetailsProps) {
  return (
    <>
      <Divider orientation="vertical" mx="xs" />
      <div
        style={{
          height: "100%",
          flexGrow: 4,
        }}
      >
        <div style={{display: "flex"}}>
          <Text weight={500}>
            {section === NavSection.Directory && "Explorer"}
            {section === NavSection.Notebooks && "Notebooks"}
            {section === NavSection.Databases && "Databases"}
            {section === NavSection.Settings && "Settings"}
          </Text>
          <div style={{flexGrow: 1}} />
          <Menu>
            <Menu.Target>
              <ActionIcon>
                <IconDots />
              </ActionIcon>
            </Menu.Target>
            <Menu.Dropdown>
              <Menu.Label>Application</Menu.Label>
              <Menu.Item icon={<IconSettings size={14} />}>Settings</Menu.Item>
              {/* TODO - Replace this... */}
            </Menu.Dropdown>
          </Menu>
        </div>
        <Divider my="xs" size={1} color="rgba(0,0,0,0.1)" />

        <ScrollArea style={{height: "100%"}} offsetScrollbars>
          {dirContents.map((d, i) => (
            d.type === "directory" ? <DirNav key={i} {...d} /> : <FileNav key={i} {...d} />
          ))}
        </ScrollArea>
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
            <Tooltip label="Explorer">
              <ActionIcon 
                onClick={() => activeSection === NavSection.Directory ? setActiveSection(undefined) : setActiveSection(NavSection.Directory)}
                color={activeSection === NavSection.Directory ? "blue" : undefined}
                variant={activeSection === NavSection.Directory ? "light" : "subtle"}
                size="xl" 
              >
                <IconFolders />
              </ActionIcon>
            </Tooltip>
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
            dirContents={dummyDirContents}
          />
        )}
      </div>
    </Navbar>
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
      styles={(theme) => ({
        main: { backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[8] : theme.colors.gray[0] },
      })}
    >
      { children }
    </AppShell>
  );
}
