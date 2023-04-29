use crate::prelude::*;

pub struct State {
    /// simulation tick
    pub tick: u32,

    /// Stores static topological info
    pub graph: Rc<Graph<GraphNode, GraphEdge>>,

    /// Internal state of agents at current tick
    pub agents: HTMap<AgentId, Agent>,

    /// Internal state of ports and markets at current tick
    pub ports: HTMap<PortId, Port>,
}

impl State {
    pub fn step(&self) -> Result<State> {
        Ok(State {
            tick: self.tick + 1,
            graph: self.graph.clone(),
            agents: self.agents.clone(),
            ports: self.ports.clone(),
        })
    }
}

struct StateHistory {
    states: Vec<State>,
}

impl StateHistory {
    pub fn step(&mut self) -> Result<()> {
        let Some(state) = self.states.last() else {
            return Err(eyre!("No last state"));
        };
        self.states.push(state.step()?);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Port {
    pub price: f32,
    pub market: Market,
}

#[derive(Debug, Clone, Default)]
pub struct GraphNode {
    name: Ustr,
    graph_idx: NodeIndex,
}

#[derive(Debug, Clone, Default)]
pub struct GraphEdge {}

#[derive(Debug, Clone, Default)]
pub struct Agent {}
