import React from "react";
import { Center, Box, Space, Title, Text, Button, Tooltip, NavLink, Aside, MediaQuery, ActionIcon, Stack, Divider, Menu, ScrollArea } from "@mantine/core";
import { IconPlus } from '@tabler/icons';


export interface IEmptyNotebookProps {
  onNew(): void;
}

export default function EmptyNotebook({onNew}: IEmptyNotebookProps) {
  return (
    <>
      <Space h="xl" />
      <Space h="xl" />
      <Space h="xl" />

      <Center>
        <Stack align="center">
          <Title order={3}>
            No Notebook Selected
          </Title>
          <Text>
            Open a notebook from the sidebar or start a new one.
          </Text>
          <Space />
          <Button 
            variant="outline"
            leftIcon={<IconPlus />}
            onClick={onNew}
          >
            New Notebook
          </Button>
        </Stack>
      </Center>
    </>
  );
}
