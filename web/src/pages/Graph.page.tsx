import { useEffect, useRef } from 'react';
import {
  interactiveNetworkBuilder,
  networkFromData,
  setColorFromData,
  setUpNetwork,
} from '@/graphics/network';
import { DefaultService } from '@/client';
import { bitFnt, makePixiApp, roundedRect } from '@/graphics/setup';
import { Assets, BitmapFont, BitmapText } from 'pixi.js';

export function Graph() {
  const ref = useRef(null);
  console.log(ref);
  useEffect(() => {
    (async () => {
      const shape = await DefaultService.networkShape();
      console.log(shape);

      const app = await makePixiApp(ref.current!);
      const containers = setUpNetwork(app);

      const networkIdMappings = networkFromData(containers, shape);

      setColorFromData({ m1: Math.random(), m2: Math.random() }, containers.nodesContainer, networkIdMappings.nodeIdToIndex);

      // interactiveNetworkBuilder(app, containers);
    })();
  }, []);

  return (
    <div style={{ width: window.innerWidth, height: window.innerHeight }}>
      <canvas ref={ref} style={{ border: '1px solid black', width: '100%', height: '100%' }} />
    </div>
  );
}
