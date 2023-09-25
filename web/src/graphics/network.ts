import { NetworkShape } from '@/client';
import {
  Application,
  Container,
  DisplayObject,
  Graphics,
  ICanvas,
  Assets,
  Rectangle,
  RoundedRectangle,
  Sprite,
  Texture,
  BitmapText,
} from 'pixi.js';

type App = Application<ICanvas> & { bg?: Sprite };
interface NetworkContainers {
  nodesContainer: Container;
  edgesContainer: Container;
  labelsContainer: Container;
}

export function setUpNetwork(app: App): NetworkContainers {
  const nodesContainer = new Container();
  const edgesContainer = new Container();
  const labelsContainer = new Container();
  app.stage.addChild(nodesContainer);
  app.stage.addChild(edgesContainer);
  app.stage.addChild(labelsContainer);
  return { nodesContainer, edgesContainer, labelsContainer };
}

export function networkFromData(
  { nodesContainer, edgesContainer, labelsContainer }: NetworkContainers,
  data: NetworkShape
) {
  const nodes = data.nodes;
  const edges = data.edges;
  const labelIdToIndex: Record<string, number> = {};
  const nodeIdToIndex: Record<string, number> = {};
  const nodeIndexToId: Record<number, string> = {};

  for (const node of nodes) {
    // todo: use a layout engine
    const x = Math.random() * 500;
    const y = Math.random() * 500;
    const nodeObj = makeNode(x, y);

    // indexing
    nodeIdToIndex[node] = nodesContainer.children.length;
    nodeIndexToId[nodesContainer.children.length] = node;

    nodesContainer.addChild(nodeObj);

    labelIdToIndex[node] = labelsContainer.children.length;
    labelsContainer.addChild(makeLabel(node, x, y));
  }

  for (const edge of edges) {
    const { u, v } = edge;
    const fromNode = nodesContainer.children[nodeIdToIndex[u]];
    const toNode = nodesContainer.children[nodeIdToIndex[v]];
    const edgeObj = makeEdge(fromNode, toNode);
    edgesContainer.addChild(edgeObj);
  }
}

export function interactiveNetworkBuilder(
  app: App,
  { nodesContainer, edgesContainer, labelsContainer }: NetworkContainers
) {
  // app.bg?.on('click', (e) => {
  //   app.stage.addChild(roundedRect(e.clientX, e.clientY, 100, 100, 10));
  // });

  let clickedNode: DisplayObject | null = null;
  app.bg?.on('click', (e) => {
    const node = makeNode(e.clientX, e.clientY);
    nodesContainer.addChild(node);
    clickedNode = node;

    node.on('click', (e) => {
      if (!clickedNode) {
        clickedNode = node;
      } else if (clickedNode !== node) {
        const edge = makeEdge(node, clickedNode);
        edgesContainer.addChild(edge);
        clickedNode = null;
      }
    });
  });
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

function makeLabel(text: string, x: number, y: number) {
  const label = new BitmapText(
    text,
    { fontName: 'Desyrel', fontSize: 20, align: 'center' }
    // node.attributes.normalizedname + ' (' + node.attributes.year + ')',
    // { font: { name: 'Arial', size: Math.max(10, node.size) }, align: 'right' }
  );
  // label.size = node.size;
  label.position.set(x, y);
  // label.anchor.set(0.5, 0.5);
  return label;
  // if (node.size <= 95) label.visible = false;
  // labels_container.addChild(label);
}

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

export async function makePixiApp(view: HTMLCanvasElement) {
  const app: App = new Application({
    view,
    resolution: window.devicePixelRatio,
    autoDensity: true,
    // backgroundColor: 0xfaafaa,
    width: window.innerWidth,
    height: window.innerHeight,
    antialias: true,
  });
  await Assets.load('https://pixijs.com/assets/bitmap-font/desyrel.xml');

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
