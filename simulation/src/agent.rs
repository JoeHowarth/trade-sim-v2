use std::ops::Index;

use enum_dispatch::enum_dispatch;
use turborand::{prelude::*, rng::Rng};

use crate::{
    behaviors::{act_greedy, act_random},
    prelude::*,
};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
#[serde(tag = "action")]
pub enum Action {
    #[default]
    Noop,
    BuyAndMove {
        good: Good,
        port_id: PortId,
    },
    Move {
        port_id: PortId,
    },
    Sell {
        good: Good,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Agent {
    pub id: AgentId,
    pub pos: PortId,
    pub cargo: Option<Good>,
    pub coins: Money,
    pub behavior: Behavior,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub enum Behavior {
    Random,
    Greedy,
    Exhaustive,
}

impl Agent {
    pub fn act(&self, ctx: &Context) -> Result<Action> {
        match self.behavior {
            Behavior::Random => act_random(self, ctx),
            Behavior::Greedy => act_greedy(self, ctx),
            Behavior::Exhaustive => act_exhaustive(self, ctx, 5),
        }
    }
}
