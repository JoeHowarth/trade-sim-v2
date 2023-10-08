//! History of the simulation
//! Used for saving output and visualization

use serde::Deserializer;

use simulation::{prelude::*, TickOutput};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct History {
    #[serde(deserialize_with = "deserialize_static_info")]
    pub static_info: &'static StaticInfo,
    pub states: Vec<State>,
    pub actions: Vec<Vec<(AgentId, Action)>>,
    pub events: Vec<Vec<Event>>,
}

impl History {
    pub fn state(&self) -> &State {
        self.states.last().unwrap()
    }

    pub fn update(&mut self, tick_output: TickOutput) {
        self.states.push(tick_output.ctx.state);
        self.actions.push(tick_output.actions);
        self.events.push(tick_output.events);
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
            static_info: StaticInfo::new_static("bar", &[("a".into(), "b".into())]),
            states: vec![State {
                tick: 0,
                agents: HTMap::default(),
                ports: HTMap::default(),
            }],
            actions: vec![vec![(
                "a".into(),
                Action::BuyAndMove {
                    good: "Wheat".into(),
                    port_id: "Genoa".into(),
                },
            )]],
            events: vec![vec![Event::Trade {
                port: "Genoa".into(),
                agent: "a".into(),
                good: "Wheat".into(),
                amt: 1,
                cost: (2.).into(),
            }]],
        };

        let serialized = serde_json::to_string(&history).unwrap();
        let deserialized: History = serde_json::from_str(&serialized).unwrap();
    }
}
