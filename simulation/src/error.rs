use crate::prelude::*;
use color_eyre::eyre;

// pub struct ApplyActionError(pub Action, pub AgentId, HTMap<PortId, Agent>, HTMap<AgentId, Agent>);
use thiserror::Error;

#[derive(Error, Clone, Debug, Serialize, Deserialize)]
#[error("Simulation Error: {e}")]
pub struct SimulationError {
    pub applied_actions: Vec<(AgentId, Action)>,
    pub unapplied_actions: Vec<(AgentId, Action)>,
    pub state: State,
    pub e: SimulationErrorKind,
}

#[derive(Error, Clone, Debug, Serialize, Deserialize)]
pub enum SimulationErrorKind {
    #[error("Invalid Action: {msg}. AgentId: {agent_id}, Action: {action:?})")]
    InvalidAction {
        action: Action,
        agent_id: AgentId,
        msg: String,
    },
}
