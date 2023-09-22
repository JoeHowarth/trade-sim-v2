pub mod exhaustive;
pub mod greedy;

pub use exhaustive::*;
pub use greedy::*;

use turborand::{rng::Rng, TurboRand};

use crate::prelude::*;

pub fn act_random(agent: &Agent, ctx: &Context) -> Result<Action> {
    let rng = Rng::default();

    let nbrs: Vec<_> = ctx
        .static_info
        .graph
        .neighbors(agent.pos)
        .collect();
    let Some(nbr) = rng
        .sample(&nbrs)
        else {
        bail!("no neighbors");
        };

    if rng.chance(0.7) {
        Ok(Action::Move { port_id: *nbr })
    } else {
        Ok(Action::Noop)
    }
}

#[cfg(test)]
mod tests {
    use crate::market::pricer::Pricer;

    use super::*;

    pub fn make_market(price: f64) -> Market {
        Market {
            table: ht_map!(Good::from("Good") => MarketInfo {
                    consumption: 1.,
                    supply: 50. - (price - 50.),
                    production: 1.,
                    pricer: Pricer::Linear {
                        base_supply: 50.,
                        base_price: 50.,
                        price_per_supply: -1.,
                    },
                }
            ),
        }
    }

    pub fn low_port() -> Port {
        Port {
            id: "low_port".into(),
            market: make_market(40.),
        }
    }

    pub fn mid_port() -> Port {
        Port {
            id: "mid_port".into(),
            market: make_market(50.),
        }
    }

    pub fn high_port() -> Port {
        Port {
            id: "high_port".into(),
            market: make_market(60.),
        }
    }

    pub fn agent() -> Agent {
        Agent {
            id: "A1".into(),
            pos: mid_port().id,
            cargo: None,
            coins: (500.).into(),
            behavior: Behavior::Greedy,
        }
    }

    pub fn ctx() -> Context {
        let mid_port = mid_port();
        let low_port = low_port();
        let high_port = high_port();
        Context {
            state: State {
                tick: 1,
                agents: ht_map!(agent().id => agent().clone()),
                ports: ht_map! {
                    high_port.id => high_port.clone(),
                    mid_port.id => mid_port.clone(),
                    low_port.id => low_port.clone()
                },
            },
            // fully connected
            static_info: StaticInfo::new_static(&[
                (mid_port.id, high_port.id),
                (mid_port.id, low_port.id),
                (high_port.id, low_port.id),
            ]),
        }
    }
}
