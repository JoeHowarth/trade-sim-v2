use serde_json::{json, Value};
use simulation::prelude::*;
use std::iter::repeat;

pub fn tabularize(history: &History) -> Result<impl Serialize> {
    let agents = tabularize_agents_json(&history)?;
    let markets = tabularize_markets_json(&history)?;
    let actions = tabularize_actions_json(&history)?;
    let events = tabularize_events_json(&history)?;
    Ok(ht_map!["agents" => agents, "markets" => markets, "actions" => actions, "events" => events])
}

fn tabularize_agents_json(history: &History) -> Result<Vec<Value>> {
    history
        .states
        .iter()
        .flat_map(|state| {
            state.agents.values().map(|agent| {
                extend_obj(
                    agent,
                    json!({
                        "tick": state.tick,
                    }),
                )
            })
        })
        .collect()
}

fn tabularize_actions_json(history: &History) -> Result<Vec<Value>> {
    history
        .actions
        .iter()
        .enumerate()
        .flat_map(|(tick, actions)| {
            actions.iter().map(move |(agent_id, action)| {
                extend_obj(
                    action,
                    json!({
                        "tick": tick,
                        "agent_id": agent_id
                    }),
                )
            })
        })
        .collect()
}

fn tabularize_events_json(history: &History) -> Result<Vec<Value>> {
    history
        .events
        .iter()
        .enumerate()
        .flat_map(|(tick, events)| {
            events.iter().map(move |event| {
                extend_obj(
                    event,
                    json!({
                        "tick": tick,
                    }),
                )
            })
        })
        .collect()
}

fn tabularize_markets_json(history: &History) -> Result<Vec<Value>> {
    let ports_with_tick = history.states.iter().flat_map(|state| {
        state
            .ports
            .values()
            .zip(repeat(state.tick.clone()))
    });
    ports_with_tick
        .flat_map(|(port, tick)| {
            port.market
                .table
                .iter()
                .map(move |(good, market)| {
                    let price = market.current_price();
                    extend_obj(
                        market,
                        json!({
                            "port": port.id,
                            "tick": tick,
                            "price": price,
                            "good": *good
                        }),
                    )
                })
        })
        .collect()
}

fn extend(row: &mut Value, mut other: Value) {
    row.as_object_mut()
        .unwrap()
        .append(other.as_object_mut().unwrap());
}

pub fn extend_obj(row: impl Serialize, other: Value) -> Result<Value> {
    let mut row = serde_json::to_value(row)?;
    extend(&mut row, other);
    Ok(row)
}
