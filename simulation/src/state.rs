use crate::prelude::*;
use serde::ser::SerializeStruct;
use std::ops::Index;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StaticInfo {
    /// Topological info
    pub graph: GraphMap<PortId, RouteId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    /// simulation tick
    pub tick: u32,

    /// Internal state of agents at current tick
    pub agents: HTMap<AgentId, Agent>,

    /// Internal state of ports and markets at current tick
    pub ports: HTMap<PortId, Port>,
}

#[derive(Clone, Debug)]
pub struct Context {
    pub state: State,
    pub static_info: &'static StaticInfo,
}

impl Context {
    pub fn step(&self) -> Result<(State, Vec<(AgentId, Action)>)> {
        // agent actions
        let actions = self
            .state
            .agents
            .iter()
            .map(|(agent_id, agent)| {
                agent.act(self).map(|action| (*agent_id, action))
            })
            .collect::<Result<Vec<(AgentId, Action)>>>()?;

        let mut ctx = self.clone();

        // process agent actions
        ctx = ctx.apply_actions(&actions)?;

        // non-agent world processes
        ctx = ctx.update_world_systems();

        Ok((ctx.state, actions))
    }

    fn apply_actions(
        self,
        actions: &[(AgentId, Action)],
    ) -> Result<Self> {
        let mut agents = self.state.agents.clone();
        let mut ports = self.state.ports.clone();

        for (agent_id, action) in actions {
            (ports, agents) =
                self.apply_action(action, agent_id, ports, agents)?;
        }

        Ok(Context {
            state: State {
                agents,
                ports,
                ..self.state.clone()
            },
            static_info: self.static_info,
        })
    }

    fn apply_action(
        &self,
        action: &Action,
        agent_id: &AgentId,
        ports: HTMap<PortId, Port>,
        agents: HTMap<AgentId, Agent>,
    ) -> Result<(HTMap<PortId, Port>, HTMap<AgentId, Agent>)> {
        let ports = ports;
        let mut agents = agents;
        match action {
            Action::Noop => {}
            Action::Move { port_id } => {
                agents = agents.try_update_with(*agent_id, |agent| {
                        if !self
                            .static_info
                            .are_neighbors(agent.pos, *port_id)
                        {
                            return Err(eyre!("Invalid Action: cannot move to a non-adjacent port"));
                        }
                        agent.pos = *port_id;
                        Ok(())
                })?;
            }
            Action::BuyAndMove { good, port_id } => {
                let mut agent = agents.index(agent_id).clone();
                let mut port = ports.index(&agent.pos).clone();
                if !self
                    .static_info
                    .are_neighbors(agent.pos, *port_id)
                {
                    return Err(eyre!("Invalid Action: cannot move to a non-adjacent port"));
                }
                agent.pos = *port_id;

                port.market.buy(good, &mut agent.coins, 1)
                        .ok_or_else(|| eyre!("Invalid Action: tried to buy when impossible"))?;
                agent.cargo = Some(*good);
                agents = agents.update(*agent_id, agent);
            }
            Action::Sell { good } => {
                let mut agent = agents.index(agent_id).clone();
                let mut port = ports.index(&agent.pos).clone();

                port.market.sell(good, &mut agent.coins, 1)
                        .ok_or_else(|| eyre!("Invalid Action: tried to sell when impossible"))?;
                agent.cargo = None;
                agents = agents.update(*agent_id, agent);
            }
        }
        Ok((ports, agents))
    }

    fn update_world_systems(&self) -> Self {
        let state = &self.state;

        let ports = state
            .ports
            .values()
            .map(|port| {
                let mut next_table = HTMap::default();
                for (good, market_info) in port.market.table.iter() {
                    let mut market_info = market_info.clone();
                    market_info.produce_and_consume();
                    next_table.insert_mut(*good, market_info);
                }
                (
                    port.id,
                    Port {
                        id: port.id,
                        market: Market { table: next_table },
                    },
                )
            })
            .collect();

        Self {
            state: State {
                ports,
                tick: state.tick + 1,
                ..state.clone()
            },
            static_info: self.static_info,
        }
    }
}

impl Context {
    pub fn new(state: State, static_info: StaticInfo) -> Self {
        Context {
            state,
            static_info: Box::leak(Box::new(static_info)),
        }
    }
}

impl StaticInfo {
    pub fn are_neighbors(&self, a: PortId, b: PortId) -> bool {
        self.graph.neighbors(a).find(|&n| n == b).is_some()
    }
    pub fn new_static(edges: &[(PortId, PortId)]) -> &'static Self {
        Box::leak(Box::new(Self::new(edges)))
    }
    pub fn new(edges: &[(PortId, PortId)]) -> Self {
        Self {
            graph: GraphMap::from_edges(edges),
        }
    }
}

impl State {
    pub fn new(ports: &[Port], agents: &[Agent]) -> Self {
        Self {
            tick: 0,
            agents: agents
                .iter()
                .map(|agent| (agent.id, agent.clone()))
                .collect(),
            ports: ports
                .iter()
                .map(|port| (port.id, port.clone()))
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Port {
    pub id: PortId,
    pub market: Market,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RouteId {}
