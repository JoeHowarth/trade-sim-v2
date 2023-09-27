import { NetworkShape } from '@/client';
import { Container, DisplayObject, Graphics, BitmapText, BitmapFont } from 'pixi.js';
import { App, bitFnt, roundedRect } from './setup';
import chroma from 'chroma-js';

export interface NetworkContainers {
  nodesContainer: Container;
  edgesContainer: Container;
}

export interface NetworkIdMappings {
  nodeIdToIndex: Record<string, number>;
}

export function setUpNetwork(app: App): NetworkContainers {
  const nodesContainer = new Container();
  const edgesContainer = new Container();

  // center in viewport
  nodesContainer.position.set(app.stage.width / 2, app.stage.height / 2);
  edgesContainer.position.set(app.stage.width / 2, app.stage.height / 2);
  app.stage.addChild(edgesContainer);
  app.stage.addChild(nodesContainer);
  return { nodesContainer, edgesContainer };
}

export function networkFromData(
  { nodesContainer, edgesContainer }: NetworkContainers,
  data: NetworkShape
): NetworkIdMappings {
  const nodes = data.nodes;
  const edges = data.edges;
  const nodeIdToIndex: Record<string, number> = {};

  const radius = 200;
  for (let i = 0; i < nodes.length; i++) {
    const nodeId = nodes[i];

    // layout in a circle
    const radians = ((Math.PI * 2) / nodes.length) * i;
    const x = radius * Math.cos(radians);
    const y = radius * Math.sin(radians);
    const nodeObj = new NetworkNode(x, y, nodeId, nodeId);

    // indexing
    nodeIdToIndex[nodeId] = nodesContainer.children.length;

    // add to containers for rendering
    nodesContainer.addChild(nodeObj);
  }

  for (const edge of edges) {
    const { u, v } = edge;
    const uNode = nodesContainer.children[nodeIdToIndex[u]];
    const vNode = nodesContainer.children[nodeIdToIndex[v]];
    edgesContainer.addChild(makeEdge(uNode, vNode));
  }
  return { nodeIdToIndex };
}

export function setColorFromData(
  nodeData: Record<string, number>,
  nodesContainer: Container,
  nodeIdToIndex: Record<string, number>
) {
  const domain = Array.from(Object.values(nodeData)).reduce(
    ([min, max], val) => [Math.min(min, val), Math.max(max, val)],
    [10000000, -1000000]
  );

  const toColor = chroma.scale(['white', 'red']).domain(domain);

  for (const id of Object.keys(nodeData)) {
    const node = nodesContainer.children[nodeIdToIndex[id]] as NetworkNode;

    node.body.tint = toColor(nodeData[id]).hex();
  }
  return domain
}

export function interactiveNetworkBuilder(
  app: App,
  { nodesContainer, edgesContainer }: NetworkContainers
) {
  let clickedNode: DisplayObject | null = null;
  app.bg?.on('click', (e) => {
    const node = new NetworkNode(e.clientX, e.clientY, Math.floor(Math.random() * 1000).toString());
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

class NetworkNode extends Container {
  id: string;
  body: Graphics;
  border: Graphics;
  label?: BitmapText;

  constructor(x: number, y: number, id: string, label?: string) {
    super();
    this.position.set(x, y);
    this.id = id;

    this.body = new Graphics();
    this.body.beginFill(0xffffff);
    this.body.drawCircle(0, 0, 50);
    this.body.endFill();
    this.body.tint = 0x202080;
    // this.body.interactive = true;

    this.border = new Graphics();
    this.border.beginFill('white');
    this.border.drawCircle(0, 0, 52);
    this.border.endFill();
    // border.alpha = 0.5;
    this.border.tint = 0x000000;

    this.addChild(this.border);
    this.addChild(this.body);
    if (label) {
      this.label = makeLabel(label, 0, 0);

      const labelBg = new Graphics();
      labelBg.beginFill('white');
      labelBg.drawRoundedRect(
        (this.label.width + 5) / -2,
        (this.label.height - 3) / -2,
        this.label.width + 5,
        this.label.height,
        5
      );
      labelBg.alpha = 0.5;

      this.addChild(labelBg);
      this.addChild(this.label);
    }

    this.interactive = true;
    this.on('click', () => console.log('clicked'));
  }
}

function makeLabel(text: string, x: number = 0, y: number = 0) {
  const label = new BitmapText(text, {
    fontName: bitFnt,
    fontSize: 20,
    align: 'center',
    tint: 'black',
  });
  label.anchor.set(0.5);
  label.position.set(x, y);
  return label;
}
