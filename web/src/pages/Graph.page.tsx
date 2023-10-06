import { useRef } from 'react';
import { MapModeSelector } from '@/components/MapModeSelector';
import { useAgentPositions, useMapMode } from '@/graphics/hooks';
import { PlaybackControls } from '@/components/PlaybackControls';
import { useReplay, useTick } from '@/components/PlaybackManager';
import { usePixiApp } from '@/graphics/setup';
import { HeaderFloating } from '@/components/HeaderFloating';
import { Button } from '@mantine/core';

export function Graph() {
  const ref = useRef(null);
  const tick = useTick();
  const replayName = useReplay();

  const { app, network, agents } = usePixiApp(ref, replayName);
  const { mapMode, setMapMode, domain } = useMapMode(network, tick, replayName);
  const [agentDataMode, setAgentDataMode] = useAgentPositions(
    app,
    network,
    agents,
    tick,
    replayName
  );

  return (
    <>
      <div
        style={{
          position: 'absolute',
          top: 0,
          right: 0,
          width: window.innerWidth,
          height: window.innerHeight,
        }}
      >
        <canvas ref={ref} style={{ width: '100%', height: '100%' }} />
      </div>
      <HeaderFloating><PlaybackControls /></HeaderFloating>
      <MapModeSelector domain={domain} mapMode={mapMode} setMapMode={setMapMode} />
      
    </>
  );
}
