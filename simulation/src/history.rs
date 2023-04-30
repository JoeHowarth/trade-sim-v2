//! History of the simulation
//! Used for saving output and visualization

use std::fmt;

use crate::prelude::*;

#[derive(Debug, Serialize)]
pub struct History {
    pub static_info: &'static StaticInfo,
    pub states: Vec<State>,
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
        let Context { state, .. } = ctx.step()?;
        self.states.push(state);
        Ok(())
    }
}

impl TryFrom<Context> for History {
    type Error = color_eyre::eyre::Error;
    fn try_from(ctx: Context) -> Result<Self> {
        if ctx.state.tick != 0 {
            return Err(eyre!("Context must be at tick 0"));
        }
        Ok(History {
            static_info: ctx.static_info,
            states: vec![ctx.state],
        })
    }
}

impl<'de> Deserialize<'de> for History {
    fn deserialize<D>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct HistoryVisitor;

        impl<'de> serde::de::Visitor<'de> for HistoryVisitor {
            type Value = History;

            fn expecting(
                &self,
                formatter: &mut fmt::Formatter,
            ) -> fmt::Result {
                panic!("here");
                formatter.write_str("struct History")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> Result<History, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut static_info = None;
                let mut states = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "static_info" => {
                            if static_info.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field(
                                        "static_info",
                                    ),
                                );
                            }
                            static_info = Some(map.next_value()?);
                        }
                        "states" => {
                            if states.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field(
                                        "states",
                                    ),
                                );
                            }
                            states = Some(map.next_value()?);
                        }
                        _ => break,
                    }
                }
                let static_info = Box::leak(Box::new(
                    static_info.ok_or_else(|| {
                        serde::de::Error::missing_field("static_info")
                    })?,
                ));
                let states = states.ok_or_else(|| {
                    serde::de::Error::missing_field("states")
                })?;
                Ok(History {
                    static_info,
                    states,
                })
            }

            fn visit_seq<A>(
                self,
                seq: A,
            ) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let _ = seq;
                panic!("hi");
            }
        }

        deserializer.deserialize_struct(
            "History",
            &["states", "static_info"],
            HistoryVisitor,
        )
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
                agents: HTMap::new(),
                ports: HTMap::new(),
            }],
        };

        let serialized = serde_json::to_string(&history).unwrap();
        let deserialized: History =
            serde_json::from_str(&serialized).unwrap();
    }
}
