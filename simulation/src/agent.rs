use std::ops::Index;

use enum_dispatch::enum_dispatch;
use turborand::{prelude::*, rng::Rng};

use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Agent {
    pub id: AgentId,
    pub pos: PortId,
    pub cargo: Option<Good>,
    pub coins: Money,
    pub behavior: Behavior,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Behavior {
    Random,
    Greedy,
}

impl Agent {
    pub fn act(&self, ctx: &Context) -> Result<Action> {
        match self.behavior {
            Behavior::Random => self.act_random(ctx),
            Behavior::Greedy => self.act_greedy(ctx),
        }
    }

    pub fn act_greedy(&self, ctx: &Context) -> Result<Action> {
        // TODO: support multiple goods
        let good = ctx
            .state
            .ports
            .values()
            .next()
            .unwrap()
            .market
            .goods()
            .next()
            .unwrap();

        // sell cargo immediately if we have any
        if let Some(cargo) = self.cargo {
            let prices = nbs_by_price(ctx, self.pos, &good);
            return Ok(Action::Sell { good: cargo });
        }

        // buy cargo and move to neighbor port with highest prices (to sell)
        if let Some((price, port_id)) =
            nbs_by_price(ctx, self.pos, good).min_by_key(|(price, _)| *price)
        {
            // continue if we can't buy
            if price < self.coins {
                return Ok(Action::BuyAndMove {
                    good: *good,
                    port_id,
                });
            }
        }

        // move to neighbor with lowest prices and try to buy next tick
        let local_price = ctx
            .state
            .ports
            .index(&self.pos)
            .market
            .price(&good);
        if let Some((price, port_id)) =
            nbs_by_price(ctx, self.pos, good).max_by_key(|(price, _)| *price)
        {
            if price < local_price {
                return Ok(Action::Move { port_id });
            }
        }

        // fall back to noop
        return Ok(Action::Noop);
    }

    pub fn act_random(&self, ctx: &Context) -> Result<Action> {
        let rng = Rng::default();

        let nbrs: Vec<_> = ctx
            .static_info
            .graph
            .neighbors(self.pos)
            .collect();
        let nbr = rng.sample(&nbrs).ok_or(eyre!("no neighbors"))?;

        if rng.chance(0.7) {
            Ok(Action::Move { port_id: *nbr })
        } else {
            Ok(Action::Noop)
        }
    }
}

fn nbs_by_price<'a>(
    ctx: &'a Context,
    port: PortId,
    good: &'a Good,
) -> impl Iterator<Item = (Money, PortId)> + 'a {
    ctx.static_info.graph.neighbors(port).map(|port| {
        (
            ctx.state
                .ports
                .get(&port)
                .unwrap()
                .market
                .price(good),
            port,
        )
    })
}
