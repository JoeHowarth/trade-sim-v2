import { DefaultService } from '@/client';
import { useState, useEffect } from 'react';
import {
  setColorFromData,
  Network,
} from './network';
import { AgentsContainer, agentsFromData } from './agents';
import { App } from './setup';


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

export function useAgentPositions(
  app: App | null,
  network: Network | null,
  agents: AgentsContainer | null,
  tick: number
) {
  const [agentDataMode, setAgentDataMode] = useState('default');

  // update positions and visualized data
  useEffect(() => {
    if (!app || !network || !agents) return;

    if (agentDataMode !== 'default') {
      console.error("Haven't implemented visually displayed agent data yet");
    }

    DefaultService.getAgentsPos(tick).then((data) => {
      agentsFromData(data, agents, network);
    });
  }, [app, network, agents, tick]);

  return [agentDataMode, setAgentDataMode];
}