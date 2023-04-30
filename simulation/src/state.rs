use serde::ser::SerializeStruct;

use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
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
        let mut state = self.clone();
        // agent actions
        let actions = state
            .agents
            .iter()
            .map(|(agent_id, agent)| {
                agent.act(self).map(|action| (*agent_id, action))
            })
            .collect::<Result<Vec<(AgentId, Action)>>>()?;

        // process actions
        state = state.apply_actions(&actions)?;

        // non-agent world processes
        // todo

        Ok(state)
    }

    fn apply_actions(
        &self,
        actions: &[(AgentId, Action)],
    ) -> Result<State> {
        let mut agents = self.agents.clone();
        let ports = self.ports.clone();

        for (agent_id, action) in actions {
            // todo: action validation
            match action {
                Action::Noop => {}
                Action::Move(port_id) => {
                    agents = agents.update_with(*agent_id, |agent| {
                        agent.pos = *port_id
                    });
                }
            }
        }
        Ok(State {
            agents,
            ports,
            ..self.clone()
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateHistory {
    pub states: Vec<State>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Port {
    pub price: f32,
    pub market: Market,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GraphNode {
    name: Ustr,
    graph_idx: NodeIndex,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GraphEdge {}
