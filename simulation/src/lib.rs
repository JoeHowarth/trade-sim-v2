#![allow(unused_imports, dead_code, incomplete_features, unused_variables)]
#![feature(return_position_impl_trait_in_trait)]
#![feature(test)]

pub mod agent;
pub mod behaviors;
pub mod error;
pub mod history;
pub mod ids;
pub mod market;
pub mod prelude;
pub mod state;

use prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Opts {
    pub ticks: u32,
}

pub fn simulation_loop(opts: Opts, history: &mut History) -> Result<()> {
    for tick in 0..opts.ticks {
        let mut ctx = Context {
            state: history.state().clone(),
            static_info: history.static_info,
        };
        let actions = fetch_agent_actions(&ctx)?;

        // apply agent actions
        let events = apply_actions(&mut ctx, &actions)?;

        // non-agent world processes
        update_world_systems(&mut ctx);

        history.update(ctx.state, actions, events)
    }

    Ok(())
}

pub fn fetch_agent_actions(ctx: &Context) -> Result<Vec<(AgentId, Action)>> {
    // todo: pluggable action api

    ctx.state
        .agents
        .iter()
        .map(|(agent_id, agent)| agent.act(ctx).map(|action| (*agent_id, action)))
        // todo: handle errors
        .collect::<Result<Vec<(AgentId, Action)>>>()
}

pub fn apply_actions(ctx: &mut Context, actions: &[(AgentId, Action)]) -> Result<Vec<Event>> {
    let mut events = Vec::with_capacity(actions.len());

    for (i, (agent_id, action)) in actions.iter().enumerate() {
        ctx.state = apply_action(&ctx, action, *agent_id, &mut events).map_err(|r| {
            let (applied_actions, unapplied_actions) = actions.split_at(i);
            let x = SimulationError {
                applied_actions: applied_actions.to_vec(),
                unapplied_actions: unapplied_actions.to_vec(),
                state: ctx.state.clone(),
                e: SimulationErrorKind::InvalidAction {
                    action: action.clone(),
                    agent_id: agent_id.clone(),
                    msg: r.to_string(),
                },
            };

            r.wrap_err(x)
        })?;
    }

    Ok(events)
}

fn apply_action(
    ctx: &Context,
    action: &Action,
    agent_id: AgentId,
    events: &mut Vec<Event>,
) -> Result<State> {
    let State {
        mut ports,
        mut agents,
        tick,
    } = ctx.state.clone();

    match action {
        Action::Noop => {}
        Action::Move { port_id } => {
            agents = agents.try_update_with(agent_id, |agent| {
                ensure!(
                    ctx.static_info.are_neighbors(agent.pos, *port_id),
                    "Cannot move to a non-adjacent port"
                );
                agent.pos = *port_id;
                Ok(())
            })?;
        }
        Action::BuyAndMove { good, port_id: dst } => {
            let mut agent = agents.get(&agent_id).unwrap().clone();
            let src = agent.pos;
            let mut port = ports.get(&src).unwrap().clone();

            // Buy
            ensure!(agent.cargo.is_none(), "Cargo must be empty to buy");
            let amt = 1;
            let cost = port
                .market
                .buy(good, &mut agent.coins, amt)
                .ok_or_else(|| eyre!("Tried to buy when impossible"))?;
            agent.cargo = Some(*good);
            events.push(Event::Trade {
                port: port.id,
                agent: agent.id,
                good: *good,
                amt,
                cost,
            });

            // Move
            ensure!(
                ctx.static_info.are_neighbors(src, *dst),
                "Cannot move to a non-adjacent port"
            );
            agent.pos = *dst;

            ports = ports.insert(src, port);
            agents = agents.insert(agent_id, agent);
        }
        Action::Sell { good } => {
            let mut agent = agents.get(&agent_id).unwrap().clone();
            let mut port = ports.get(&agent.pos).unwrap().clone();

            // Sell
            ensure!(
                matches!(agent.cargo, Some(good)),
                "Agent must have matching cargo to sell"
            );
            let amt = -1;
            let cost = port
                .market
                .sell(good, &mut agent.coins, -amt)
                .ok_or_else(|| eyre!("Tried to sell when impossible"))?;
            agent.cargo = None;
            events.push(Event::Trade {
                port: port.id,
                agent: agent.id,
                good: *good,
                amt,
                cost,
            });

            ports = ports.insert(port.id, port);
            agents = agents.insert(agent_id, agent);
        }
    }
    Ok(State {
        tick: tick,
        ports,
        agents,
    })
}

pub fn update_world_systems(ctx: &mut Context) {
    ctx.state.ports = ctx
        .state
        .ports
        .values()
        .map(|port| {
            let mut next_table = HTMap::default();
            for (good, market_info) in port.market.table.iter() {
                let mut market_info = market_info.clone();
                market_info.produce_and_consume();
                next_table.insert_mut(*good, market_info);
            }
            (
                port.id,
                Port {
                    id: port.id,
                    market: Market { table: next_table },
                },
            )
        })
        .collect();
}
