use serde_json::{json, Value};
use simulation::{prelude::*, TickOutput};
use std::iter::repeat;

use crate::history::History;

pub fn tabularize_history(history: &History) -> Result<impl Serialize> {
    let agents = tabularize_agents_history(&history)?;
    let markets = tabularize_markets_history(&history)?;
    let actions = tabularize_actions_history(&history)?;
    let events = tabularize_events_history(&history)?;
    Ok(ht_map!["agents" => agents, "markets" => markets, "actions" => actions, "events" => events])
}

pub fn tabularize_tick_output(o: TickOutput) -> Result<impl Serialize> {
    let agents = tabularize_agents_state(&o.ctx.state).collect::<Result<Vec<_>>>()?;
    let markets = tabularize_markets_state(&o.ctx.state).collect::<Result<Vec<_>>>()?;
    let actions = tabularize_actions_state((o.ctx.state.tick as usize, &o.actions))
        .collect::<Result<Vec<_>>>()?;
    let events = tabularize_events_state((o.ctx.state.tick as usize, &o.events))
        .collect::<Result<Vec<_>>>()?;
    Ok(ht_map!["agents" => agents, "markets" => markets, "actions" => actions, "events" => events])
}

pub fn tabularize_agents_state<'a>(state: &'a State) -> impl Iterator<Item = Result<Value>> + 'a {
    state.agents.values().map(|agent| {
        extend_obj(
            agent,
            json!({
                "tick": state.tick,
            }),
        )
    })
}

fn tabularize_agents_history(history: &History) -> Result<Vec<Value>> {
    history
        .states
        .iter()
        .flat_map(tabularize_agents_state)
        .collect()
}

pub fn tabularize_actions_state<'a>(
    (tick, actions): (usize, &'a Vec<(AgentId, Action)>),
) -> impl Iterator<Item = Result<Value>> + 'a {
    actions.iter().map(move |(agent_id, action)| {
        extend_obj(
            action,
            json!({
                "tick": tick,
                "agent_id": agent_id
            }),
        )
    })
}

fn tabularize_actions_history(history: &History) -> Result<Vec<Value>> {
    history
        .actions
        .iter()
        .enumerate()
        .flat_map(tabularize_actions_state)
        .collect()
}

pub fn tabularize_events_state<'a>(
    (tick, events): (usize, &'a Vec<Event>),
) -> impl Iterator<Item = Result<Value>> + 'a {
    events.iter().map(move |event| {
        extend_obj(
            event,
            json!({
                "tick": tick,
            }),
        )
    })
}

fn tabularize_events_history(history: &History) -> Result<Vec<Value>> {
    history
        .events
        .iter()
        .enumerate()
        .flat_map(tabularize_events_state)
        .collect()
}

pub fn tabularize_markets_state<'a>(state: &'a State) -> impl Iterator<Item = Result<Value>> + 'a {
    state
        .ports
        .values()
        .zip(repeat(state.tick.clone()))
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
}

fn tabularize_markets_history(history: &History) -> Result<Vec<Value>> {
    history
        .states
        .iter()
        .flat_map(tabularize_markets_state)
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
