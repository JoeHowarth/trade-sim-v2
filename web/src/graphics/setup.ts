import {
  Application,
  Graphics,
  ICanvas,
  Assets,
  Sprite,
  Texture,
  BitmapFont,
  Container,
} from 'pixi.js';

export type App = Application<ICanvas> & { bg: Sprite; centered: Container };

export function roundedRect(
  x: number,
  y: number,
  width: number,
  height: number,
  radius: number = 5
) {
  const obj = new Graphics();
  obj.position.set(x, y);

  obj.beginFill(0x000050);
  obj.drawRoundedRect(-width / 2, -height / 2, width, height, radius);
  obj.endFill();

  obj.interactive = true;
  obj.on('click', () => {
    obj.rotation += 0.1;
  });
  return obj;
}

export const bitFnt = 'bitFnt';

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
  app.stage.addChild(bg);
  app.bg = bg;

  // center on middle of screen
  const centered = new Container();
  app.stage.addChild(centered);
  app.centered = centered;
  centered.position.set(app.screen.width / 2, app.screen.height / 2);

  app.centered.addChild(roundedRect(0, 0, 10, 10));
  console.log(app.centered.children[0].getGlobalPosition());

  return app;
}
