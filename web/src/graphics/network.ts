import {
  Application,
  Container,
  DisplayObject,
  Graphics,
  ICanvas,
  Rectangle,
  RoundedRectangle,
  Sprite,
  Texture,
} from 'pixi.js';

type App = Application<ICanvas> & { bg?: Sprite };

export function network(view: HTMLCanvasElement) {
  const app = makeApp(view);

  // app.bg?.on('click', (e) => {
  //   app.stage.addChild(roundedRect(e.clientX, e.clientY, 100, 100, 10));
  // });

  app.stage.addChild(roundedRect(50, 50, 100, 100, 10));

  const nodesContainer = new Container();
  const edgesContainer = new Container();

  let clickedNode: DisplayObject | null = null;

  app.bg?.on('click', (e) => {
    const node = makeNode(e.clientX, e.clientY);
    nodesContainer.addChild(node);
    clickedNode = node;

    node.on('click', (e) => {
      if (!clickedNode) {
        clickedNode = node;
      } else {
        const edge = makeEdge(node, clickedNode);
        edgesContainer.addChild(edge);
        clickedNode = null;
      }
    });
  });

  app.stage.addChild(nodesContainer);
  app.stage.addChild(edgesContainer);
}

function makeEdge(from: DisplayObject, to: DisplayObject) {
  const edge = new Graphics();
  edge.lineStyle(10, 0x202080);
  edge.moveTo(from.x, from.y);
  edge.lineTo(to.x, to.y);
  return edge;
}

function makeNode(x: number, y: number) {
  const node = new Graphics();
  node.position.set(x, y);
  node.beginFill(0x202080);
  node.drawCircle(0, 0, 50);
  node.endFill();
  node.interactive = true;
  return node;
}

function roundedRect(x: number, y: number, width: number, height: number, radius: number = 5) {
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

function makeApp(view: HTMLCanvasElement) {
  const app: App = new Application({
    view,
    resolution: window.devicePixelRatio,
    autoDensity: true,
    // backgroundColor: 0xfaafaa,
    width: window.innerWidth,
    height: window.innerHeight,
    antialias: true
  });

  let bg = new Sprite(Texture.WHITE);
  // Set it to fill the screen
  bg.width = app.screen.width;
  bg.height = app.screen.height;
  // Add a click handler
  bg.interactive = true;
  app.stage.addChild(bg);
  app.bg = bg;
  return app;
}
