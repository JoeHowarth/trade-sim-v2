use crate::{market::Money, prelude::*};

#[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq, Clone)]
pub enum Pricer {
    Linear {
        base_supply: f64,
        base_price: f64,
        price_per_supply: f64,
    },
    Inverse {
        coef: f64,
    },
}

impl Pricer {
    pub fn price(&self, amt: f64) -> Money {
        match self {
            Pricer::Linear {
                base_supply,
                base_price,
                price_per_supply,
            } => {
                let mut p = price_per_supply * (amt as f64 - base_supply) + base_price;
                if p < 0. {
                    warn!("Negative price, clipping to 0");
                    p = 0.
                }
                p.into()
            }
            Pricer::Inverse { coef } => Money::from(coef / amt),
        }
    }

    pub fn linear(base_supply: f64, base_price: f64, price_per_supply: f64) -> Self {
        if price_per_supply > 0. {
            panic!(
                "Expected price per supply to be negative, actually: {:?}",
                price_per_supply
            );
        }
        Pricer::Linear {
            base_price,
            base_supply,
            price_per_supply,
        }
    }

    pub fn inverse(coef: impl Into<f64>) -> Pricer {
        Pricer::Inverse { coef: coef.into() }
    }
}

mod tests {
    use super::*;

    #[test]
    fn linear_pricer() {
        let pricer = Pricer::linear(50., 10., -2.);
        assert_eq!(pricer.price(51.), 8.0.into());
    }

    #[test]
    fn constant_product_pricer() {
        let pricer = Pricer::inverse(10);
        assert_eq!(pricer.price(10.), 1.0.into());
    }
}
