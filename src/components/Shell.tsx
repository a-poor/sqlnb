import React, { useState } from "react";
import { AppShell, Navbar, Text, Tooltip, NavLink, ActionIcon, Stack, Divider, Menu, ScrollArea } from "@mantine/core";
import { IconFolders, IconFolder, IconFile, IconNotebook, IconDatabase, IconSettings, IconDots } from '@tabler/icons';


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

function FileNavItem({name, size}: IFileData) {
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

function DirNavItem({name, contents}: IDirectoryData) {
  return (
    <NavLink 
      label={name}
      icon={<IconFolder size={16} stroke={1.5}/>}
      childrenOffset={7}
    >
      {contents.map((d, i) => (
        d.type === "directory" ? <DirNavItem key={i} {...d} /> : <FileNavItem key={i} {...d} />
      ))}

    </NavLink>
  );
}

interface INavDetailsBaseProps {
  title: string;
  menu?: React.ReactNode; // Dropdown menu items...
  children: React.ReactNode;
}

function NavDetailsBase({title, menu, children}: INavDetailsBaseProps) {
  return (
    <>
      <div style={{display: "flex"}}>
        <Text weight={500}>
          { title }
        </Text>
        <div style={{flexGrow: 1}} />
        { menu }
      </div>
      <Divider my="xs" size={1} color="rgba(0,0,0,0.1)" />
      <ScrollArea style={{height: "100%"}} offsetScrollbars>
        { children }
      </ScrollArea>
    </>
  );
}

interface INavExplorerDetailsProps {
  dirContents: (IDirectoryData | IFileData)[];
}

function NavExplorerDetails({dirContents}: INavExplorerDetailsProps) {
  return (
    <NavDetailsBase
      title="Explorer"
      menu={
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
      }
    >
      {dirContents.map((d, i) => (
        d.type === "directory" ? <DirNavItem key={i} {...d} /> : <FileNavItem key={i} {...d} />
      ))}
    </NavDetailsBase>
  );
}

interface INavNotebookDetailsProps {
  notebooks: {
    name: string;
    status: "running" | "stopped";
  }[];
}

function NavNotebookDetails({notebooks}: INavNotebookDetailsProps) {
  return (
    <NavDetailsBase
      title="Notebooks"
      menu={
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
      }
    >
      {/* {notebooks.map((d, i) => (
        d.type === "directory" ? <DirNavItem key={i} {...d} /> : <FileNavItem key={i} {...d} />
      ))} */}
    </NavDetailsBase>
  );
}

interface INavDbConnectionDetailsProps {
  connections: {
    name: string;
    type: "sqlite" | "postgres";
    // TODO - Add more details...
  }[];
}

function NavDbConnectionDetails({connections}: INavDbConnectionDetailsProps) {
  return (
    <NavDetailsBase
      title="DB Connections"
      menu={
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
      }
    >
      {/* {notebooks.map((d, i) => (
        d.type === "directory" ? <DirNavItem key={i} {...d} /> : <FileNavItem key={i} {...d} />
      ))} */}
    </NavDetailsBase>
  );
}

interface INavSettingsDetailsProps {
  // TODO - Add more details...
}

function NavSettingsDetails({}: INavSettingsDetailsProps) {
  return (
    <NavDetailsBase
      title="Settings"
      menu={
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
      }
    >
      {/* TODO: ... */}
    </NavDetailsBase>
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
        {section === NavSection.Directory && <NavExplorerDetails dirContents={dirContents} />}
        {section === NavSection.Notebooks && <NavNotebookDetails notebooks={[]} />}
        {section === NavSection.Databases && <NavDbConnectionDetails connections={[]} />}
        {section === NavSection.Settings && <NavSettingsDetails />}
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
            <Tooltip label="Notebooks">
              <ActionIcon 
                onClick={() => activeSection === NavSection.Notebooks ? setActiveSection(undefined) : setActiveSection(NavSection.Notebooks)}
                color={activeSection === NavSection.Notebooks ? "blue" : undefined}
                variant={activeSection === NavSection.Notebooks ? "light" : "subtle"}
                size="xl" 
              >
                <IconNotebook />
              </ActionIcon>
            </Tooltip>
            <Tooltip label="DB Connections">
              <ActionIcon 
                onClick={() => activeSection === NavSection.Databases ? setActiveSection(undefined) : setActiveSection(NavSection.Databases)}
                color={activeSection === NavSection.Databases ? "blue" : undefined}
                variant={activeSection === NavSection.Databases ? "light" : "subtle"}
                size="xl" 
              >
                <IconDatabase />
              </ActionIcon>
            </Tooltip>
            <Tooltip label="Settings">
              <ActionIcon 
                onClick={() => activeSection === NavSection.Settings ? setActiveSection(undefined) : setActiveSection(NavSection.Settings)}
                color={activeSection === NavSection.Settings ? "blue" : undefined}
                variant={activeSection === NavSection.Settings ? "light" : "subtle"}
                size="xl" 
              >
                <IconSettings />
              </ActionIcon>
            </Tooltip>
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
