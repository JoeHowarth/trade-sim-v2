import { DefaultService } from '@/client';
import { useState, useEffect } from 'react';
import {
  NetworkIdMappings,
  NetworkContainers,
  networkFromData,
  setColorFromData,
  Network,
} from './network';
import { App, makePixiApp } from './setup';
import { AgentsContainer } from './agents';
import { Container } from 'pixi.js';

export function usePixiApp(ref: React.MutableRefObject<HTMLCanvasElement | null>) {
  const [app, setApp] = useState<App | null>(null);
  const [network, setNetwork] = useState<null | Network>(null);
  const [agents, setAgentsContainer] = useState<null | AgentsContainer>(null);

  useEffect(() => {
    (async () => {
      const [app, shape] = await Promise.all([
        makePixiApp(ref.current!),
        DefaultService.networkShape(),
      ]);

      const network = new Network(app.centered);
      networkFromData(shape, network);
      const agentsContainer = new AgentsContainer(app.centered);
      setAgentsContainer(agentsContainer);
      setNetwork(network);
      setApp(app);
    })();
  }, []);

  return { app, network, agents };
}

export function useMapMode(network: Network | null, tick: number) {
  const [mapMode, setMapMode] = useState('default');
  const [domain, setDomain] = useState<[number, number]>([0, 1]);

  useEffect(() => {
    if (!network) return;

    (async () => {
      let data: Record<string, number> = {};
      if (mapMode === 'default') {
        for (const key of Object.keys(network.nodeIdToIndex)) {
          data[key] = 1;
        }
      } else {
        data = await DefaultService.marketCol(tick, mapMode);
      }

      setDomain(setColorFromData(data, network));
    })();
  }, [mapMode, network, tick]);

  return { mapMode, setMapMode, domain };
}
