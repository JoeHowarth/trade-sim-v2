import { useEffect, useRef, useState } from 'react';
import {
  NetworkContainers,
  NetworkIdMappings,
  interactiveNetworkBuilder,
  networkFromData,
  setColorFromData,
  setUpNetwork,
} from '@/graphics/network';
import { DefaultService } from '@/client';
import { bitFnt, makePixiApp, roundedRect } from '@/graphics/setup';
import { Assets, BitmapFont, BitmapText } from 'pixi.js';
import { Text, Box, Center, Paper, SegmentedControl, Space, Stack, Title } from '@mantine/core';

export function Graph() {
  const ref = useRef(null);
  const [networkIdMappings, setNetworkIdMappings] = useState<null | NetworkIdMappings>(null);
  const [containers, setContainers] = useState<null | NetworkContainers>(null);
  const [mapMode, setMapMode] = useState('default');
  const [domain, setDomain] = useState([0, 1]);

  useEffect(() => {
    (async () => {
      const shape = await DefaultService.networkShape();
      console.log(shape);

      const app = await makePixiApp(ref.current!);
      const containers = setUpNetwork(app);

      const networkIdMappings = networkFromData(containers, shape);

      setNetworkIdMappings(networkIdMappings);
      setContainers(containers);
    })();
  }, []);

  useEffect(() => {
    if (!networkIdMappings || !containers) return;

    (async () => {
      let data: Record<string, number> = {};
      if (mapMode === 'default') {
        for (const key of Object.keys(networkIdMappings.nodeIdToIndex)) {
          data[key] = 1;
        }
      } else {
        data = await DefaultService.marketCol(0, mapMode);
      }
      const domain = setColorFromData(
        data,
        containers.nodesContainer,
        networkIdMappings.nodeIdToIndex
      );
      setDomain(domain);
    })();
  }, [mapMode]);

  return (
    <>
      <div style={{ width: window.innerWidth, height: window.innerHeight }}>
        <canvas ref={ref} style={{ border: '1px solid black', width: '100%', height: '100%' }} />
      </div>
      <Paper
        style={{ zIndex: 3, position: 'absolute', bottom: 0, right: 0 }}
        shadow="xs"
        radius="xs"
        p="md"
        m="sm"
        withBorder
      >
        <Stack align="center">
          <Title order={4}>Map Mode</Title>
          <Text>{`${Math.round(domain[0])} to ${Math.round(domain[1])}`}</Text>
          <SegmentedControl
            orientation="vertical"
            value={mapMode}
            onChange={setMapMode}
            data={['Default', 'Price', 'Supply', 'Consumption', 'Production'].map((x) => ({
              label: x,
              value: x.toLowerCase(),
            }))}
          />
        </Stack>
      </Paper>
    </>
  );
}
