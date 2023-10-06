import { useState } from 'react';
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
    <HeaderFloating
      center={
        <Group gap={5} visibleFrom="xs">
          <Button variant="light" color="rgb(117, 140, 161)">
            Run Scenario
          </Button>
          <Button variant="light" color="rgba(117, 140, 161, 1)">
            View Replay
          </Button>
        </Group>
      }
      right={<Burger opened={true} size="sm" />}
    ></HeaderFloating>
  );
}

export function HeaderFloating({ center, right }: any) {
  const navigate = useNavigate();
  const goHome = () => navigate('/');

  return (
    <Paper shadow="xs" style={{ width: 'fit-content', height: 50, border: '1px solid #eee' }}>
      <Group style={{ height: '100%', paddingLeft: 5, paddingRight: 5 }}>
        <ActionIcon onClick={goHome} variant="transparent" color="black">
          <IconSailboat size={28} />
        </ActionIcon>
        <Title style={{ cursor: 'pointer' }} onClick={goHome} order={3}>
          Trade Sim v2
        </Title>
        <Space w="md" />
        {center}
        {right ? <Space w="md" /> : null}
        {right}
      </Group>
    </Paper>
    // </header>
  );
}
