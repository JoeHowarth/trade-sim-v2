import { useRef } from 'react';
import { MapModeSelector } from '@/components/MapModeSelector';
import { useAgentPositions, useMapMode } from '@/graphics/hooks';
import { PlaybackControls } from '@/components/PlaybackControls';
import { useTick } from '@/components/PlaybackManager';
import { usePixiApp } from '@/graphics/setup';

export function Graph() {
  const ref = useRef(null);
  const tick = useTick();

  const { app, network, agents } = usePixiApp(ref);
  const { mapMode, setMapMode, domain } = useMapMode(network, tick);
  const [agentDataMode, setAgentDataMode] = useAgentPositions(app, network, agents, tick);

  return (
    <>
      <div style={{ width: window.innerWidth, height: window.innerHeight }}>
        <canvas ref={ref} style={{ width: '100%', height: '100%' }} />
      </div>
      <MapModeSelector domain={domain} mapMode={mapMode} setMapMode={setMapMode} />
      <PlaybackControls />
    </>
  );
}
