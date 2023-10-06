// import { HeaderSimple } from '@/components/HeaderSimple/HeaderSimple';
import { ReplayInfo, ReplayService, ScenarioService, Scenario_Output } from '@/client';
import {
  ActionIcon,
  Button,
  ButtonGroup,
  Card,
  Group,
  JsonInput,
  Paper,
  Popover,
  Space,
  Stack,
  Tabs,
  TextInput,
  Title,
} from '@mantine/core';
import { useCounter } from '@mantine/hooks';
import { IconArrowLeft } from '@tabler/icons-react';
import { PropsWithChildren, useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';

export function HomePage() {
  const navigate = useNavigate();

  return (
    <CardOnBackground title="Trade Sim v2">
      <Stack p="md" gap="xs">
        <Button
          onClick={() => navigate('/run-scenario')}
          color="rgb(117, 140, 161)"
          variant="light"
        >
          Run Scenario
        </Button>
        <Button
          onClick={() => navigate('/view-replay')}
          color="rgba(117, 140, 161, 1)"
          variant="light"
        >
          View Replay
        </Button>
      </Stack>
    </CardOnBackground>
  );
}

export function ViewReplay() {
  const navigate = useNavigate();
  const [replays, setReplays] = useState<null | string[]>(null);
  const [replay, setReplay] = useState<null | string>(null);
  const [replayInfo, setReplayInfo] = useState<null | ReplayInfo>(null);

  useEffect(() => {
    ReplayService.replayall().then(setReplays);
  }, []);

  useEffect(() => {
    if (replay) {
      ReplayService.replaygetInfo(replay).then((info) => {
        setReplayInfo(info);
      });
    }
  }, [replay]);

  const btns = replays?.map((r) => (
    <Button
      key={r}
      onClick={() => setReplay((old) => (r == old ? null : r))}
      color="rgb(117, 140, 161)"
      variant={r == replay ? 'filled' : 'light'}
    >
      {r}
    </Button>
  ));

  return (
    <CardOnBackground title="View Replay" back={() => navigate('/')}>
      <Group align="flex-start">
        <Stack justify="flex-start" p="md" gap="xs">
          {btns}
        </Stack>
        {replayInfo ? (
          <Stack gap="xs">
            <p style={{ margin: 0 }}>Num Ticks: {replayInfo.ticks}</p>
            <Button
              onClick={() => {
                navigate('/' + replayInfo.name + '/0');
              }}
              variant="light"
            >
              Run
            </Button>
          </Stack>
        ) : null}
      </Group>
    </CardOnBackground>
  );
}

export function RunScenarioPage() {
  const navigate = useNavigate();
  const [scenarios, setScenarios] = useState<null | string[]>(null);
  const [selectedScenario, setSelectedScenario] = useState<null | string>(null);
  const [scenarioData, setScenarioData] = useState<Scenario_Output | null>(null);
  const [stringScenarioData, setStringScenarioData] = useState<string>('');
  const [count, handlers] = useCounter();

  useEffect(() => {
    ScenarioService.scenarioall().then(setScenarios);
  }, [count]);

  useEffect(() => {
    if (selectedScenario) {
      ScenarioService.scenarioget(selectedScenario).then((x) => {
        setScenarioData(x);
        setStringScenarioData(JSON.stringify(x, undefined, 2));
      });
    }
  }, [selectedScenario, count]);

  const btns = scenarios?.map((scenario) => (
    <Button
      key={scenario}
      onClick={() => setSelectedScenario((old) => (scenario == old ? null : scenario))}
      color="rgb(117, 140, 161)"
      variant={scenario == selectedScenario ? 'filled' : 'light'}
    >
      {scenario}
    </Button>
  ));

  return (
    <CardOnBackground title="Run Scenario" back={() => navigate('/')}>
      <Group align="flex-start">
        <Stack justify="flex-start" p="md" gap="xs">
          {btns}
        </Stack>
        {selectedScenario && scenarioData ? (
          <ScenarioViewer
            scenarioName={selectedScenario}
            scenarioData={scenarioData}
            stringScenarioData={stringScenarioData}
            setStringScenarioData={setStringScenarioData}
            setScenarioData={setScenarioData}
            onSaveAs={(name) => {
              ScenarioService.scenariopost(name, scenarioData).then(() => handlers.increment());
            }}
          />
        ) : null}
      </Group>
    </CardOnBackground>
  );
}

function ScenarioViewer({
  scenarioName,
  scenarioData,
  stringScenarioData,
  setStringScenarioData,
  setScenarioData,
  onSaveAs,
}: {
  scenarioName: string;
  scenarioData: Scenario_Output;
  stringScenarioData: string;
  setStringScenarioData: (str: string) => void;
  setScenarioData: (x: Scenario_Output) => void;
  onSaveAs: (name: string) => void;
}) {
  let numGoods = 0;
  let numPorts = 0;
  if (scenarioData.ports.length > 0) {
    numGoods = Object.keys(scenarioData.ports[0].market.table).length;
    numPorts = scenarioData.ports.length;
  }

  return (
    <Tabs defaultValue="info">
      <Tabs.List>
        <Tabs.Tab value="info">Info</Tabs.Tab>
        <Tabs.Tab value="json">Edit Json</Tabs.Tab>
      </Tabs.List>
      <Tabs.Panel value="info">
        <Stack gap="xs">
          <p style={{ margin: 0 }}>Num Agents: {scenarioData?.agents.length}</p>
          <p style={{ margin: 0 }}>Num Ports: {numPorts}</p>
          <p style={{ margin: 0 }}>Num Goods: {numGoods}</p>
          <Button variant="light">Run</Button>
        </Stack>
      </Tabs.Panel>
      <Tabs.Panel value="json">
        <Stack>
          <JsonInput
            minRows={10}
            maxRows={40}
            formatOnBlur
            autosize
            value={stringScenarioData}
            onChange={(str) => {
              setStringScenarioData(str);
              setScenarioData(JSON.parse(str));
            }}
            style={{ width: '300px' }}
          />
          <Button onClick={() => onSaveAs(scenarioName)} variant="light">
            Save
          </Button>
          <Popover width={300} trapFocus position="bottom" withArrow shadow="md">
            <Popover.Target>
              <Button variant="light">Save As</Button>
            </Popover.Target>
            <Popover.Dropdown>
              <TextInput
                onKeyDown={(e) => {
                  if (e.key == 'Enter') {
                    onSaveAs(e.currentTarget.value.trim());
                  }
                }}
                label="Name"
                placeholder="Name"
                size="xs"
              />
            </Popover.Dropdown>
          </Popover>
        </Stack>
      </Tabs.Panel>
    </Tabs>
  );
}

export function CardOnBackground({
  title,
  children,
  back,
}: PropsWithChildren<{ title: string; back?: () => void }>) {
  return (
    <>
      <div style={{ width: '100vw', height: '100vh', background: 'rgb(250, 249, 250)' }}></div>
      <Card
        style={{ zIndex: 3, position: 'absolute', top: 0, left: 0 }}
        shadow="md"
        radius="sm"
        p="md"
        m="sm"
        withBorder
      >
        <Card.Section withBorder inheritPadding py="xs">
          <Group>
            {back && (
              <ActionIcon onClick={back} variant="transparent" color="black">
                <IconArrowLeft />
              </ActionIcon>
            )}
            <Title order={1}>{title}</Title>
          </Group>
        </Card.Section>
        {children}
      </Card>
    </>
  );
}

export function MyPaper({ children }: PropsWithChildren) {
  return (
    <Paper
      style={{ zIndex: 3, position: 'absolute', top: 0, left: 0 }}
      shadow="md"
      radius="sm"
      p="md"
      m="sm"
      withBorder
    >
      {children}
    </Paper>
  );
}
