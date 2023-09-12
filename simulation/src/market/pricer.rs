use crate::{market::Money, prelude::*};

pub trait Pricer {
    /// Cost of buying 1 unit when supply is `amt`
    fn price(&self, amt: f64) -> Money;
}

#[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq, Clone)]
pub struct LinearPricer {
    pub base_supply: f64,
    pub base_price: f64,
    pub price_per_supply: f64,
}

impl Pricer for LinearPricer {
    fn price(&self, amt: f64) -> Money {
        let p = self.price_per_supply * (amt as f64 - self.base_supply) + self.base_price;
        if p < 0. {
            warn!("Negative price, clipping to 0");
            Money::from(0.)
        } else {
            Money::from(p)
        }
    }
}

impl LinearPricer {
    pub fn new(base_supply: f64, base_price: f64, price_per_supply: f64) -> Self {
        if price_per_supply > 0. {
            panic!(
                "Expected price per supply to be negative, actually: {:?}",
                price_per_supply
            );
        }
        LinearPricer {
            base_price,
            base_supply,
            price_per_supply,
        }
    }
}

mod tests {
    use crate::market::{
        self,
        exchanger::{Exchanger, MarketInfo},
        pricer::{LinearPricer, Pricer},
        Money,
    };

    #[test]
    fn basic() {
        let pricer = LinearPricer::new(50., 10., -2.);
        assert_eq!(pricer.price(51.), 8.0.into());
    }
}
