import { DataService } from '@/client';
import { useState, useEffect } from 'react';
import { setColorFromData, Network, networkFromData } from './network';
import { AgentsContainer, agentsFromData } from './agents';
import { App } from './setup';

export function useMapMode(network: Network | null, tick: number, replayName: string) {
  const [mapMode, setMapMode] = useState('default');
  const [domain, setDomain] = useState<[number, number]>([0, 1]);

  useEffect(() => {
    if (!network) return;

    (async () => {
      const data = await DataService.datanetworkShape(replayName);
      networkFromData(data, network);
    })();
  }, [replayName, network]);

  useEffect(() => {
    if (!network) return;

    (async () => {
      let data: Record<string, number> = {};
      if (mapMode === 'default') {
        for (const key of Object.keys(network.nodeIdToIndex)) {
          data[key] = 1;
        }
      } else {
        data = await DataService.datamarketCol(tick, mapMode, replayName);
      }

      setDomain(setColorFromData(data, network));
    })();
  }, [mapMode, network, tick, replayName]);

  return { mapMode, setMapMode, domain };
}

export function useAgentPositions(
  app: App | null,
  network: Network | null,
  agents: AgentsContainer | null,
  tick: number,
  replayName: string
) {
  const [agentDataMode, setAgentDataMode] = useState('default');

  // update positions and visualized data
  useEffect(() => {
    if (!app || !network || !agents) return;

    if (agentDataMode !== 'default') {
      console.error("Haven't implemented visually displayed agent data yet");
    }

    DataService.datagetAgentsPos(tick, replayName).then((data) => {
      agentsFromData(data, agents, network);
    });
  }, [app, network, agents, tick, replayName]);

  return [agentDataMode, setAgentDataMode];
}
