use std::default::Default;

use rpds::ht_map;
use simulation::prelude::*;

#[test]
fn test_state_serde() {
    let market_info = MarketInfo {
        supply: 100.,
        pricer: pricer::LinearPricer::new(1., 1., 1.),
        consumption: 80.,
        production: 20.,
    };
    let state1 = State::new(
        &[
            Port {
                id: "Genoa".into(),
                market: Market {
                    table: Default::default(),
                },
            },
            Port {
                id: "Rome".into(),
                market: Market {
                    table: ht_map![
                                "Wheat".into() => market_info.clone(),
                                "Iron".into() => market_info.clone()
                    ],
                },
            },
        ],
        &[Agent {
            id: "A".into(),
            pos: "Genoa".into(),
        }],
        &[("Genoa".into(), "Rome".into())],
    );

    let json = serde_json::to_string_pretty(&state1).unwrap();
    println!("{}", json);
    let state2: State = serde_json::from_str(&json).unwrap();
    println!("{:?}", state1);
    println!("{:?}", state2);
}
