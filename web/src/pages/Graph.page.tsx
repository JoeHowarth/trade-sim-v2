import { useEffect, useRef } from 'react';
import { MapModeSelector } from '@/components/MapModeSelector';
import { useMapMode, useNetwork, usePixiApp } from '@/graphics/hooks';
import { DefaultService } from '@/client';
import { agentsFromData, setUpAgents } from '@/graphics/agents';

export function Graph() {
  const ref = useRef(null);

  const app = usePixiApp(ref);
  const network = useNetwork(app);
  const { mapMode, setMapMode, domain } = useMapMode(network);

  useEffect(() => {
    if (!app || !network) return;
    (async () => {
      const agentsContainer = setUpAgents(app);
      const data = await DefaultService.getAgentsPos(0);
      agentsFromData(data, agentsContainer, network);
    })();
  }, [app, network]);

  return (
    <>
      <div style={{ width: window.innerWidth, height: window.innerHeight }}>
        <canvas ref={ref} style={{ border: '1px solid black', width: '100%', height: '100%' }} />
      </div>
      <MapModeSelector domain={domain} mapMode={mapMode} setMapMode={setMapMode} />
    </>
  );
}
