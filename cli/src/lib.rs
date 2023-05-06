use std::{iter::repeat, path::PathBuf};

use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use simulation::prelude::*;

pub fn extract_agents_json(history: &History) -> Result<Vec<Value>> {
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

pub fn extract_actions_json(history: &History) -> Result<Vec<Value>> {
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

pub fn extract_markets_json(history: &History) -> Result<Vec<Value>> {
    let ports_with_tick = history.states.iter().flat_map(|state| {
        state.ports.values().zip(repeat(state.tick.clone()))
    });
    ports_with_tick
        .flat_map(|(port, tick)| {
            port.market.table.iter().map(move |(good, market)| {
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

pub fn extend_obj(
    row: impl Serialize,
    other: Value,
) -> Result<Value> {
    let mut row = serde_json::to_value(row)?;
    extend(&mut row, other);
    Ok(row)
}

pub fn load_json_file<T: DeserializeOwned>(
    path: impl Into<PathBuf>,
) -> Result<T> {
    serde_json::from_reader(std::io::BufReader::new(
        std::fs::File::open(path.into())?,
    ))
    .map_err(Into::into)
}

pub fn save_json_file(
    path: impl Into<PathBuf>,
    json: impl Serialize,
) -> Result<()> {
    serde_json::to_writer_pretty(
        std::io::BufWriter::new(std::fs::File::create(path.into())?),
        &json,
    )?;
    Ok(())
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
