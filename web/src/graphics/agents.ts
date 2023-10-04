import { AgentInfo } from '@/client';
import { Container, Graphics, BitmapText } from 'pixi.js';
import { App, bitFnt } from './setup';
import chroma from 'chroma-js';
import { Network } from './network';
import { circleLayout, getDomain } from './utils';
import { useState } from 'react';

type AgentUpdateData = { id: string; pos: string; data: number; label?: string };

export class AgentsContainer extends Container {
  agentIdToIndex: Record<string, number> = {};

  constructor(parent: Container) {
    super();
    parent.addChild(this);
  }

  getAgent(id: string): Agent {
    return this.children[this.agentIdToIndex[id]] as Agent;
  }

  addAgent(agent: Agent) {
    this.agentIdToIndex[agent.id] = this.children.length;
    this.addChild(agent);
  }
}

export class Agent extends Container {
  circle: Graphics;
  text: BitmapText;
  id: string;

  constructor(x: number, y: number, id: string, label: string) {
    super();
    this.position.set(x, y);
    this.id = id;

    this.circle = new Graphics();
    this.circle.beginFill(0x000000);
    this.circle.drawCircle(0, 0, 10);
    this.circle.endFill();
    this.circle.tint = 'green';
    this.addChild(this.circle);

    // should use background to make this more readable
    this.text = new BitmapText(label, { fontName: bitFnt, fontSize: 12, tint: 'white' });
    this.text.anchor.set(0.5, 0.5);
    this.text.position.set(0, 0);
    this.addChild(this.text);
  }
}

export function agentsFromData(
  agents: Record<string, AgentInfo>,
  agentsContainer: AgentsContainer,
  network: Network
) {
  const byPort = groupbyPort(Object.values(agents));
  const radius = 61;
  agentsLayout(byPort, network, radius, ({ id }, x, y) => {
    let agentObj = agentsContainer.getAgent(id);
    if (!agentObj) {
      agentObj = new Agent(x, y, id, id);
      agentsContainer.addAgent(agentObj);
    } else {
      agentObj.position.set(x, y);
    }
  });
}

function groupbyPort<A extends { pos: string; id: string }>(agents: A[]): Record<string, A[]> {
  const byPort = {} as Record<string, A[]>;
  for (const agent of agents) {
    const { pos } = agent;
    if (byPort[pos] === undefined) {
      byPort[pos] = [];
    }
    byPort[pos].push(agent);
  }
  return byPort;
}

function agentsLayout<A extends { pos: string; id: string }>(
  byPort: Record<string, A[]>,
  network: Network,
  radius: number,
  fn: (agent: A, x: number, y: number) => void
) {
  for (const port of Object.keys(byPort)) {
    const node = network.getNode(port);
    const localAgents = byPort[port];
    for (let i = 0; i < localAgents.length; i++) {
      const [dx, dy] = circleLayout(i, localAgents.length, radius);
      const x = dx + node.x;
      const y = dy + node.y;
      fn(localAgents[i], x, y);
    }
  }
}

// export function updateAgents(
//   agentData: AgentUpdateData[],
//   agentsContainer: AgentContainer,
//   network: Network
// ): [number, number] {
//   const domain = getDomain(agentData, (a) => a.data);
//   const toColor = chroma.scale(['white', 'red']).domain(domain);

//   const byPort = groupbyPort(agentData);
//   const radius = 61;
//   agentsLayout(byPort, network, radius, (agent, x, y) => {
//     const agentObj = agentsContainer.children.find(
//       (child) => (child as Agent).id === agent.id
//     ) as Agent;

//     agentObj.position.set(x, y);
//     agentObj.circle.tint = toColor(agent.data).hex();
//   });

//   return domain;
// }
