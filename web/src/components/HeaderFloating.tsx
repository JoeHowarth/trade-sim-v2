import { PropsWithChildren, ReactNode, useState } from 'react';
import {
  Container,
  Group,
  Burger,
  Paper,
  Button,
  Flex,
  Space,
  Title,
  ActionIcon,
} from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
// import classes from './HeaderSimple.module.css';
import { IconSailboat } from '@tabler/icons-react';
import { hex } from 'chroma-js';
import { useNavigate } from 'react-router-dom';

export function ExHeader() {
  return (
    <HeaderFloating right={<Burger opened={true} size="sm" />}>
      <Group gap={5} visibleFrom="xs">
        <Button variant="light" color="rgb(117, 140, 161)">
          Run Scenario
        </Button>
        <Button variant="light" color="rgba(117, 140, 161, 1)">
          View Replay
        </Button>
      </Group>
    </HeaderFloating>
  );
}

export function HeaderFloating({ children, right }: PropsWithChildren<{ right?: ReactNode }>) {
  const navigate = useNavigate();
  const goHome = () => navigate('/');

  return (
    <Paper
      shadow="xs"
      style={{
        width: 'fit-content',
        height: 50,
        border: '1px solid #eee',
        zIndex: 3,
        position: 'absolute',
        top: 0,
        left: 0,
      }}
    >
      <Group style={{ height: '100%', paddingLeft: 5, paddingRight: 5 }}>
        <ActionIcon onClick={goHome} variant="transparent" color="black">
          <IconSailboat size={28} />
        </ActionIcon>
        <Title style={{ cursor: 'pointer' }} onClick={goHome} order={2}>
          Trade Sim v2
        </Title>
        <Space w="md" />
        {children}
        {right ? (
          <>
            <Space w="md" />
            {right}
          </>
        ) : null}
      </Group>
    </Paper>
    // </header>
  );
}
