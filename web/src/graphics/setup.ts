import { Application, ICanvas, Sprite, Texture, BitmapFont, Container } from 'pixi.js';
import { useState, useEffect } from 'react';
import { AgentsContainer } from './agents';
import { Network, networkFromData } from './network';
import { DataService } from '@/client';

export type App = Application<ICanvas> & { bg: Sprite; centered: Container };

export const bitFnt = 'bitFnt';

export function usePixiApp(
  ref: React.MutableRefObject<HTMLCanvasElement | null>,
  replayName: string
) {
  const [app, setApp] = useState<App | null>(null);
  const [network, setNetwork] = useState<null | Network>(null);
  const [agents, setAgentsContainer] = useState<null | AgentsContainer>(null);

  useEffect(() => {
    (async () => {
      const [app, shape] = await Promise.all([
        makePixiApp(ref.current!),
        DataService.datanetworkShape(replayName),
      ]);
      setApp(app);

      const network = new Network(app.centered);
      networkFromData(shape, network);
      setNetwork(network);
      setAgentsContainer(new AgentsContainer(app.centered));
    })();
    return () => {
      console.log('Running pixi cleanup...');
      setApp((app) => {
        if (!app) return null;
        app.destroy();
        return null;
      });
      setNetwork(null);
      setAgentsContainer(null);
      console.log('Pixi cleanup complete.');
    };
  }, []);

  return { app, network, agents, replayName };
}

export async function makePixiApp(view: HTMLCanvasElement) {
  const app = new Application({
    view,
    resolution: window.devicePixelRatio,
    autoDensity: true,
    width: window.innerWidth,
    height: window.innerHeight,
    antialias: true,
  }) as App;

  // load bitmap fonts
  BitmapFont.from(bitFnt, { fill: 'white' });

  let bg = new Sprite(Texture.WHITE);
  // Set it to fill the screen
  bg.width = app.screen.width;
  bg.height = app.screen.height;
  // Add a click handler
  bg.interactive = true;
  bg.tint = 0xf6f7f8;
  app.stage.addChild(bg);
  app.bg = bg;

  // center on middle of screen
  const centered = new Container();
  app.stage.addChild(centered);
  app.centered = centered;
  centered.position.set(app.screen.width / 2, app.screen.height / 2);

  return app;
}
