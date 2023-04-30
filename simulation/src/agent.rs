use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum Action {
    #[default]
    Noop,
    Move(PortId),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Agent {
    pub id: AgentId,
    pub pos: PortId,
}

impl Agent {
    pub fn act(&self, state: &State) -> Result<Action> {
        Ok(Action::Noop)
    }
}
