use crate::{
    market::{
        money::Money,
        pricer::{Pricer},
    },
    prelude::*,
    PortId,
};
use std::ops::DerefMut;

#[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq, Clone)]
pub struct MarketInfo {
    pub consumption: f64,
    pub supply: f64,
    pub production: f64,
    pub pricer: Pricer,
}

impl MarketInfo {
    pub fn cost(&self, amt: i32) -> Money {
        // Invariant: cost of buying followed by selling the same number must sum to 0.
        if amt == 0 {
            return 0.0.into();
        }
        let price = |amt| self.pricer.price(amt);
        let avg_price = if amt > 0 {
            (price(self.supply) + price(self.supply - amt as f64)) / 2.
        } else {
            (price(self.supply) + price(self.supply - amt as f64)) / 2.
        };
        (avg_price * amt as f64).into()
    }

    /// `buy` takes a mutable `wallet` and an amount, `amt`, to buy and performs the transaction if possible
    /// if cost is greater than contents of wallet, return None
    /// the cost of the transaction is removed from `wallet` and the cost is returned
    /// the supply of goods is decreased by `amt`
    pub fn buy(&mut self, wallet: &mut Money, amt: i32) -> Option<Money> {
        if amt == 0 {
            return Some(0.0.into());
        }
        if self.supply < amt as f64 {
            error!("self.supply < amt");
            return None;
        }
        let cost = self.cost(amt);
        if cost > *wallet {
            error!("cost {cost} > wallet {wallet}");
            return None;
        }
        *wallet -= cost;
        self.supply -= amt as f64;
        Some(cost)
    }

    pub fn sell(&mut self, wallet: &mut Money, amt: i32) -> Option<Money> {
        self.buy(wallet, -amt)
    }
}

impl MarketInfo {
    pub fn current_price(&self) -> Money {
        self.pricer.price(self.supply)
    }
    pub fn produce_and_consume(&mut self) -> &mut Self {
        self.supply = self.supply + self.production - self.consumption;
        self
    }
}

mod tests {
    use super::*;

    #[test]
    fn cost() {
        let pricer = Pricer::linear(35., 100., -1.);
        let market_info = MarketInfo {
            consumption: 0.,
            supply: 35.,
            production: 0.,
            pricer: pricer.clone(),
        };
        assert_eq!(
            market_info.cost(1),
            (market_info.pricer.price(market_info.supply)
                + market_info.pricer.price(market_info.supply - 1.))
                / 2.
        );
        assert_eq!(market_info.cost(1), Money::from(100.5));
        assert_eq!(
            market_info.cost(-1),
            (market_info.pricer.price(market_info.supply)
                + market_info.pricer.price(market_info.supply + 1.))
                / -2.
        );
        assert_eq!(market_info.cost(-1), Money::from(-99.5));

        assert_eq!(
            market_info.cost(10),
            10. * (market_info.pricer.price(market_info.supply)
                + market_info.pricer.price(market_info.supply - 10.))
                / 2.
        );
        assert_eq!(market_info.cost(10), Money::from(10. * (100. + 110.) / 2.));
        assert_eq!(market_info.cost(-10), Money::from(-10. * (100. + 90.) / 2.));
    }

    #[test]
    fn buy_sell() {
        let pricer = Pricer::linear(35., 100., -1.);
        let mut market_info = MarketInfo {
            consumption: 30.,
            supply: 35.,
            production: 29.,
            pricer: pricer.clone(),
        };

        let starting_balance = Money(10_000.);
        for &amt in [1., 10.].iter() {
            let mut wallet: Money = starting_balance;
            let initial_cost = market_info.cost(amt as i32);
            let initial_money = Some(initial_cost.into());

            assert_eq!(market_info.buy(&mut wallet, amt as i32), initial_money);
            assert_eq!(wallet, starting_balance - initial_money.unwrap());

            market_info.buy(&mut wallet, amt as i32 * 2);
            market_info.sell(&mut wallet, amt as i32 * 2);

            assert_eq!(
                market_info.sell(&mut wallet, amt as i32),
                initial_money.map(Money::rneg)
            );
            assert_eq!(wallet, starting_balance);
        }
    }
}
