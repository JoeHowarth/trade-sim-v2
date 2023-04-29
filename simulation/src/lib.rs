#![allow(unused_imports, dead_code)]

pub mod market;
pub mod prelude;

use market::Market;
use prelude::*;

struct State {
    /// simulation tick
    pub tick: u32,

    /// Stores static topological info
    pub graph: Rc<Graph<GraphNode, GraphEdge>>,

    /// Internal state of agents at current tick
    pub agents: HashTrieMap<AgentId, Agent>,

    /// Internal state of ports and markets at current tick
    pub ports: HashTrieMap<PortId, Port>,
}

#[derive(Debug, Clone, Default)]
struct Port {
    pub price: f32,
    pub market: Market,
}

#[derive(Debug, Clone, Default)]
struct GraphNode {
    name: Ustr,
    graph_idx: NodeIndex,
}

#[derive(Debug, Clone, Default)]
struct GraphEdge {}

#[derive(Debug, Clone, Default)]
struct Agent {}

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    Deref,
    DerefMut,
    From,
    PartialEq,
    Eq,
    Hash,
)]
struct PortId(pub Ustr);

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    Deref,
    DerefMut,
    From,
    PartialEq,
    Eq,
    Hash,
)]
struct AgentId(pub Ustr);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
