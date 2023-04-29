pub mod exchanger;
pub mod linear_market;
pub mod money;
pub mod pricer;

use crate::prelude::*;
use std::fmt::Debug;

pub use self::{
    exchanger::{Exchanger, MarketInfo},
    money::Money,
    pricer::Pricer,
};

#[derive(From, Debug, Clone)]
pub struct Market {
    pub table: HTMap<Good, MarketInfo>,
}

impl Market {
    fn price(&self, good: &Good) -> Money {
        let info = self.info(good);
        info.pricer.price(info.supply)
    }

    fn cost(&self, good: &Good, amt: i32) -> Money {
        self.info(good).cost(amt)
    }

    fn goods(&self) -> impl Iterator<Item = &Good> {
        self.table.keys()
    }

    fn info(&self, good: &Good) -> &exchanger::MarketInfo {
        self.table
            .get(&good)
            .expect(&*format!("Good: {} not found in market", *good))
    }

    fn info_mut(
        &mut self,
        good: &Good,
    ) -> &mut exchanger::MarketInfo {
        self.table
            .get_mut(&good)
            .expect(&*format!("Good: {} not found in market", *good))
    }

    fn buy(
        &mut self,
        good: &Good,
        wallet: &mut Money,
        amt: i32,
    ) -> Option<Money> {
        self.info_mut(good).buy(wallet, amt)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn linear_market_cost() {
        let pricer = pricer::LinearPricer::new(35., 100., -1.);
        let market_info = MarketInfo {
            consumption: 30.,
            supply: 35.,
            production: 29.,
            pricer: pricer.clone(),
        };
        let market_info_after = MarketInfo {
            supply: 30.,
            ..market_info.clone()
        };
        let (before, after) =
            (Good::from("Before"), Good::from("After"));
        let lm: Market = [
            (before.clone(), market_info.clone()),
            (after.clone(), market_info_after.clone()),
        ]
        .iter()
        .cloned()
        .collect::<HTMap<Good, MarketInfo>>()
        .into();

        let five_times_current_price: Money =
            market_info.current_price() * 5.;
        assert!(
            lm.cost(&before, 5) > five_times_current_price,
            "cost of buying 5 should be more than 5*current_price to avoid buy/sell arbitrage"
        );
        assert_eq!(
            lm.cost(&before, 2),
            pricer.price(35.) + pricer.price(34.)
        );
        assert_eq!(
            lm.cost(&before, 5),
            std::iter::repeat(35.)
                .enumerate()
                .map(|(i, s)| pricer.price(s - i as f64))
                .take(5)
                .sum::<Money>()
        );
        assert_eq!(
            lm.cost(&before, -2),
            -(pricer.price(36.) + pricer.price(37.))
        );

        assert_eq!(
            lm.cost(&after, -2),
            -(pricer.price(31.) + pricer.price(32.))
        );
        assert_eq!(lm.cost(&before, 5), -lm.cost(&after, -5));
    }

    fn goods() -> impl Iterator<Item = Good> {
        ["Wood", "Iron", "Food"]
            .iter()
            .map::<&str, _>(AsRef::as_ref)
            .map(Good::from)
    }

    #[test]
    fn linear_market_basics() {
        let base_price = 100.;
        let good = goods().next().unwrap();
        let pricer = pricer::LinearPricer::new(35., base_price, -1.);
        let base_supply = 35.;
        let market_info = MarketInfo {
            consumption: 30.,
            supply: base_supply,
            production: 29.,
            pricer: pricer.clone(),
        };
        let lm: Market = goods()
            .map(|gh| (gh, market_info.clone()))
            .collect::<HTMap<Good, MarketInfo>>()
            .into();

        // .price(good)
        assert_eq!(lm.price(&good), pricer.price(base_supply));
        // .goods()
        assert_eq!(
            lm.goods().cloned().collect::<HTSet<Good>>(),
            goods().collect::<HTSet<Good>>()
        );
        // .info(good)
        assert_eq!(lm.info(&good), &market_info)
    }
}
