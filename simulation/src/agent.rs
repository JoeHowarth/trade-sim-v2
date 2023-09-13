use std::ops::Index;

use enum_dispatch::enum_dispatch;
use turborand::{prelude::*, rng::Rng};

use crate::prelude::*;

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
        let current_port = ctx.state.ports.g(self.pos);
        // TODO: support multiple goods
        let good = current_port.market.goods().next().unwrap();

        // sell cargo immediately if we have any
        if let Some(cargo) = self.cargo {
            return Ok(Action::Sell { good: cargo });
        }
        let local_price = current_port.market.price(good);

        // buy cargo and move to neighbor port with highest prices (to sell)
        if let Some((max_nbr_price, port_id)) =
            nbs_by_price(ctx, self.pos, good).max_by_key(|(price, _)| *price)
        {
            let can_sell_profitably = max_nbr_price > current_port.market.price(good);
            let can_buy_here = local_price < self.coins;

            if can_buy_here && can_sell_profitably {
                return Ok(Action::BuyAndMove {
                    good: *good,
                    port_id,
                });
            }
            // otherwise continue
        }

        // move to neighbor with lowest prices and try to buy next tick
        if let Some((price, port_id)) =
            nbs_by_price(ctx, self.pos, good).min_by_key(|(price, _)| *price)
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

mod tests {
    use crate::market::pricer::LinearPricer;

    use super::*;

    fn make_market(price: f64) -> Market {
        Market {
            table: ht_map!(Good::from("Good") => MarketInfo {
                    consumption: 1.,
                    supply: 50. - (price - 50.),
                    production: 1.,
                    pricer: LinearPricer {
                        base_supply: 50.,
                        base_price: 50.,
                        price_per_supply: -1.,
                    },
                }
            ),
        }
    }

    fn milan() -> Port {
        Port {
            id: "Milan".into(),
            market: make_market(40.),
        }
    }

    fn genoa() -> Port {
        Port {
            id: "Genoa".into(),
            market: make_market(50.),
        }
    }

    fn rome() -> Port {
        Port {
            id: "Rome".into(),
            market: make_market(60.),
        }
    }

    fn agent() -> Agent {
        Agent {
            id: "A1".into(),
            pos: genoa().id,
            cargo: None,
            coins: (500.).into(),
            behavior: Behavior::Greedy,
        }
    }

    fn ctx() -> Context {
        let genoa = genoa();
        let milan = milan();
        let rome = rome();
        Context {
            state: State {
                tick: 1,
                agents: ht_map!(agent().id => agent().clone()),
                ports: ht_map! {
                    rome.id => rome.clone(),
                    genoa.id => genoa.clone(),
                    milan.id => milan.clone()
                },
            },
            // fully connected
            static_info: StaticInfo::new_static(&[
                (genoa.id, rome.id),
                (genoa.id, milan.id),
                (rome.id, milan.id),
            ]),
        }
    }

    #[test]
    fn buy_and_move_from_lowest() {
        let good = Good::from("Good");
        let genoa = genoa();
        let milan = milan();
        let rome = rome();
        let agent = agent();
        let ctx = ctx();

        assert!(rome.market.price(&good) > genoa.market.price(&good));
        assert!(rome.market.cost(&good, 1) < agent.coins);

        // Since agent does not have any goods, it should buy in genoa and move to rome since it has the highest price
        let action = agent.act(&ctx).unwrap();
        assert_eq!(
            action,
            Action::BuyAndMove {
                good,
                port_id: rome.id
            }
        );
    }

    #[test]
    fn buy_and_move_from_middle() {
        let good = Good::from("Good");
        let genoa = genoa();
        let milan = milan();
        let rome = rome();
        let agent = Agent {
            pos: genoa.id,
            ..agent()
        };
        let ctx = ctx();

        assert!(rome.market.price(&good) > genoa.market.price(&good));
        assert!(rome.market.cost(&good, 1) < agent.coins);

        // Since agent does not have any goods, it should buy in genoa and move to rome since it has the highest price
        let action = agent.act(&ctx).unwrap();
        assert_eq!(
            action,
            Action::BuyAndMove {
                good,
                port_id: rome.id
            }
        );
    }

    #[test]
    fn move_with_nothing_to_lowest_to_buy() {
        let good = Good::from("Good");
        let genoa = genoa();
        let milan = milan();
        let rome = rome();
        let agent = Agent {
            pos: rome.id,
            ..agent()
        };
        let ctx = ctx();

        assert!(rome.market.price(&good) > genoa.market.price(&good));
        assert!(rome.market.cost(&good, 1) < agent.coins);

        // Since agent does not have any goods, it should buy in genoa and move to rome since it has the highest price
        let action = agent.act(&ctx).unwrap();
        assert_eq!(
            action,
            Action::Move { port_id: milan.id }
        );
    }
}
