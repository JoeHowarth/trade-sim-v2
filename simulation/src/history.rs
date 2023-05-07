//! History of the simulation
//! Used for saving output and visualization

use std::fmt;

use serde::Deserializer;

use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct History {
    #[serde(deserialize_with = "deserialize_static_info")]
    pub static_info: &'static StaticInfo,
    pub states: Vec<State>,
    pub actions: Vec<Vec<(AgentId, Action)>>,
}

impl History {
    pub fn state(&self) -> &State {
        self.states.last().unwrap()
    }

    pub fn push(&mut self, ctx: Context) {
        if (ctx.state.tick as usize) < self.states.len() {
            panic!("Tick {} already exists", ctx.state.tick)
        }
        if (ctx.state.tick as usize) > self.states.len() {
            panic!(
                "Tick {} not next, last: {}",
                ctx.state.tick,
                self.states.last().unwrap().tick
            );
        }
        self.states.push(ctx.state);
    }

    pub fn step(&mut self) -> Result<()> {
        let ctx = Context {
            state: self.state().clone(),
            static_info: self.static_info,
        };
        let (state, actions) = ctx.step()?;
        self.states.push(state);
        self.actions.push(actions);
        Ok(())
    }
}

/// WARNING: this leaks memory
fn deserialize_static_info<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<&'static StaticInfo, D::Error> {
    match StaticInfo::deserialize(deserializer) {
        Ok(static_info) => Ok(Box::leak(Box::new(static_info))),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_round_trip() {
        let history = History {
            static_info: StaticInfo::new_static(&[(
                "a".into(),
                "b".into(),
            )]),
            states: vec![State {
                tick: 0,
                agents: HTMap::default(),
                ports: HTMap::default(),
            }],
            actions: vec![vec![]],
        };

        let serialized = serde_json::to_string(&history).unwrap();
        let deserialized: History =
            serde_json::from_str(&serialized).unwrap();
    }
}
