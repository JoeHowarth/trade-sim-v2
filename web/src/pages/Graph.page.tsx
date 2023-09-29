import { useEffect, useRef } from 'react';
import { MapModeSelector } from '@/components/MapModeSelector';
import { useMapMode, usePixiApp } from '@/graphics/hooks';
import { DefaultService } from '@/client';
import { agentsFromData, AgentsContainer } from '@/graphics/agents';
import { PlaybackControls } from '@/components/PlaybackControls';
import { useTick } from '@/components/PlaybackManager';

export function Graph() {
  const ref = useRef(null);
  const tick = useTick();

  const { app, network, agents } = usePixiApp(ref);
  // const network = useNetwork(app);
  // const agents = useAgents(app);
  const { mapMode, setMapMode, domain } = useMapMode(network, tick);

  useEffect(() => {
    if (!app || !network || !agents) return;

    DefaultService.getAgentsPos(tick).then((data) => {
      agentsFromData(data, agents, network);
    });
  }, [app, network, agents, tick]);

  return (
    <>
      <div style={{ width: window.innerWidth, height: window.innerHeight }}>
        <canvas ref={ref} style={{ border: '1px solid black', width: '100%', height: '100%' }} />
      </div>
      <MapModeSelector domain={domain} mapMode={mapMode} setMapMode={setMapMode} />
      <PlaybackControls />
    </>
  );
}
