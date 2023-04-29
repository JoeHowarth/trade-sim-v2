
pub struct State {
    /// simulation tick
    pub tick: u32,

    /// Stores static topological info
    pub graph: Rc<Graph<GraphNode, GraphEdge>>,

    /// Internal state of agents at current tick
    pub agents: HashTrieMap<AgentId, Agent>,

    /// Internal state of ports and markets at current tick
    pub ports: HashTrieMap<PortId, Port>,
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
