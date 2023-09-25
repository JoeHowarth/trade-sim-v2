import { useEffect, useRef } from 'react';
import {
  interactiveNetworkBuilder,
  makePixiApp,
  networkFromData,
  roundedRect,
  setUpNetwork,
} from '@/graphics/network';
import { DefaultService } from '@/client';

export function Graph() {
  const ref = useRef(null);
  console.log(ref);
  useEffect(() => {
    (async () => {
      const service = new DefaultService();
      const shape = await DefaultService.networkShape();
      console.log(shape);

      const app = await makePixiApp(ref.current!);
      app.stage.addChild(roundedRect(50, 50, 100, 100, 10));
      const containers = setUpNetwork(app);

      networkFromData(containers, shape)
      interactiveNetworkBuilder(app, containers);
    })();
  }, []);

  return (
    <div style={{ width: window.innerWidth, height: window.innerHeight }}>
      <canvas ref={ref} style={{ border: '1px solid black', width: '100%', height: '100%' }} />
    </div>
  );
}
