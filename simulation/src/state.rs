use crate::{error::SimulationErrorKind, prelude::*};
use serde::ser::SerializeStruct;
use std::{borrow::Cow, ops::Index, str::FromStr};

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
            .map(|(agent_id, agent)| agent.act(self).map(|action| (*agent_id, action)))
            // todo: handle errors
            .collect::<Result<Vec<(AgentId, Action)>>>()?;

        let mut ctx = self.clone();

        // apply agent actions
        ctx = ctx.apply_actions(&actions)?;

        // non-agent world processes
        ctx = ctx.update_world_systems();

        Ok((ctx.state, actions))
    }

    fn apply_actions(self, actions: &[(AgentId, Action)]) -> Result<Self> {
        let mut state = self.state.clone();

        for (i, (agent_id, action)) in actions.iter().enumerate() {
            state = self
                .apply_action(action, agent_id, state.clone())
                .map_err(|r| {
                    let (applied_actions, unapplied_actions) = actions.split_at(i);
                    let x = SimulationError {
                        applied_actions: applied_actions.to_vec(),
                        unapplied_actions: unapplied_actions.to_vec(),
                        state,
                        e: SimulationErrorKind::InvalidAction {
                            action: action.clone(),
                            agent_id: agent_id.clone(),
                            msg: r.to_string(),
                        },
                    };

                    r.wrap_err(x)
                })?;
        }

        Ok(Context {
            state,
            static_info: self.static_info,
        })
    }

    fn apply_action(&self, action: &Action, agent_id: &AgentId, state: State) -> Result<State> {
        let State {
            mut ports,
            mut agents,
            tick,
        } = state;

        match action {
            Action::Noop => {}
            Action::Move { port_id } => {
                agents = agents.try_update_with(*agent_id, |agent| {
                    ensure!(
                        self.static_info
                            .are_neighbors(agent.pos, *port_id),
                        "Cannot move to a non-adjacent port"
                    );
                    agent.pos = *port_id;
                    Ok(())
                })?;
            }
            Action::BuyAndMove { good, port_id } => {
                let logic = |agent: &mut Agent, port: &mut Port| -> Result<()> {
                    ensure!(
                        self.static_info
                            .are_neighbors(agent.pos, *port_id),
                        "Cannot move to a non-adjacent port"
                    );
                    agent.pos = *port_id;

                    port.market
                        .buy(good, &mut agent.coins, 1)
                        .ok_or_else(|| eyre!("Tried to buy when impossible"))?;
                    agent.cargo = Some(*good);

                    Ok(())
                };

                // update
                agents = agents.try_update_with(*agent_id, |agent| {
                    ports = ports.try_update_with(agent.pos, |port| logic(agent, port))?;
                    Ok(())
                })?;
            }
            Action::Sell { good } => {
                let logic = |agent: &mut Agent, port: &mut Port| -> Result<()> {
                    port.market
                        .sell(good, &mut agent.coins, 1)
                        .ok_or_else(|| eyre!("Tried to sell when impossible"))?;
                    agent.cargo = None;
                    Ok(())
                };

                // update
                agents = agents.try_update_with(*agent_id, |agent| {
                    ports = ports.try_update_with(agent.pos, |port| logic(agent, port))?;
                    Ok(())
                })?;
            }
        }
        Ok(State {
            tick: tick,
            ports,
            agents,
        })
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
        self.graph
            .neighbors(a)
            .find(|&n| n == b)
            .is_some()
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
