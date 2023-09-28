import { NetworkShape } from '@/client';
import { Container, DisplayObject, Graphics, BitmapText } from 'pixi.js';
import { App, bitFnt } from './setup';
import chroma from 'chroma-js';
import { circleLayout, getDomain } from './utils';

export class Network {
  nodesContainer: Container;
  edgesContainer: Container;
  nodeIdToIndex: Record<string, number>;

  constructor(
    nodesContainer: Container,
    edgesContainer: Container,
    nodeIdToIndex: Record<string, number>
  ) {
    this.nodesContainer = nodesContainer;
    this.edgesContainer = edgesContainer;
    this.nodeIdToIndex = nodeIdToIndex;
  }

  getNode(id: string): NetworkNode {
    return this.nodesContainer.children[this.nodeIdToIndex[id]] as NetworkNode;
  }

  addNode(node: NetworkNode) {
    this.nodeIdToIndex[node.id] = this.nodesContainer.children.length;
    this.nodesContainer.addChild(node);
  }

  addEdge(from: NetworkNode, to: NetworkNode, edge: Graphics) {
    from.addEdge(to, edge);
    to.addEdge(from, edge);
    this.edgesContainer.addChild(edge);
  }
}

export interface NetworkContainers {
  nodesContainer: Container;
  edgesContainer: Container;
}

export interface NetworkIdMappings {
  nodeIdToIndex: Record<string, number>;
}

export function setUpNetwork(app: App): Network {
  const nodesContainer = new Container();
  const edgesContainer = new Container();

  // center in viewport
  app.centered.addChild(edgesContainer);
  app.centered.addChild(nodesContainer);
  return new Network(nodesContainer, edgesContainer, {});
}

export function networkFromData(data: NetworkShape, network: Network) {
  const nodes = data.nodes;
  const edges = data.edges;

  const radius = 200;
  for (let i = 0; i < nodes.length; i++) {
    const nodeId = nodes[i];

    // layout in a circle
    const [x, y] = circleLayout(i, nodes.length, radius);
    network.addNode(new NetworkNode(x, y, nodeId, nodeId));
  }

  for (const { u, v } of edges) {
    const uNode = network.getNode(u);
    const vNode = network.getNode(v);
    network.addEdge(uNode, vNode, makeEdge(uNode, vNode));
  }
}

export function setColorFromData(
  nodeData: Record<string, number>,
  network: Network
): [number, number] {
  const domain = getDomain(Object.values(nodeData));

  const toColor = chroma.scale(['white', 'red']).domain(domain);

  for (const id of Object.keys(nodeData)) {
    const node = network.getNode(id);
    node.body.tint = toColor(nodeData[id]).hex();
  }
  return domain as [number, number];
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
  edges: { node: NetworkNode; edge: DisplayObject }[] = [];

  addEdge(node: NetworkNode, edge: DisplayObject) {
    this.edges.push({ node, edge });
  }

  edge(nodeId: string): DisplayObject | undefined {
    return this.edges.find(({ node }) => node.id === nodeId)?.edge;
  }

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

      // todo: make this part of label class
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

// old

export function interactiveNetworkBuilder(
  app: App,
  { nodesContainer, edgesContainer }: NetworkContainers
) {
  let clickedNode: DisplayObject | null = null;
  app.centered?.on('click', (e) => {
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
