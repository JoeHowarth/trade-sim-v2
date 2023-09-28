import { DefaultService } from '@/client';
import { useState, useEffect } from 'react';
import {
  NetworkIdMappings,
  NetworkContainers,
  setUpNetwork,
  networkFromData,
  setColorFromData,
  Network,
} from './network';
import { App, makePixiApp } from './setup';

export function usePixiApp(ref: React.MutableRefObject<HTMLCanvasElement | null>) {
  const [app, setApp] = useState<App | null>(null);

  useEffect(() => {
    makePixiApp(ref.current!).then(setApp);
  }, []);

  return app;
}

export function useNetwork(app: App | null) {
  const [network, setNetwork] = useState<null | Network>(null);

  useEffect(() => {
    if (!app) return;

    (async () => {
      const shape = await DefaultService.networkShape();
      const network = setUpNetwork(app);

      networkFromData(shape, network);
      setNetwork(network);
    })();
  }, [app]);

  return network;
}

export function useMapMode(network: Network | null) {
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
        data = await DefaultService.marketCol(0, mapMode);
      }

      setDomain(setColorFromData(data, network));
    })();
  }, [mapMode, network]);

  return { mapMode, setMapMode, domain };
}
