use crate::{error::SimulationErrorKind, prelude::*};
use serde::ser::SerializeStruct;
use std::{borrow::Cow, ops::Index, str::FromStr};

#[derive(Clone, Debug)]
pub struct Context {
    pub state: State,
    pub static_info: &'static StaticInfo,
}

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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum Event {
    Trade {
        port: PortId,
        agent: AgentId,
        good: Good,
        amt: i32,
        cost: Money,
    },
}

impl Context {
    pub fn new(state: State, static_info: StaticInfo) -> Self {
        Context {
            state,
            static_info: Box::leak(Box::new(static_info)),
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

pub fn nbs_with_price<'a>(
    ctx: &'a Context,
    port: PortId,
    good: &'a Good,
) -> impl Iterator<Item = (Money, PortId)> + 'a {
    ctx.static_info
        .graph
        .neighbors(port)
        .map(|port| (ctx.state.ports.g(port).market.price(good), port))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Port {
    pub id: PortId,
    pub market: Market,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RouteId {}
