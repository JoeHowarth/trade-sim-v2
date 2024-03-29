#![allow(unused_imports, dead_code, incomplete_features, unused_variables)]
#![feature(return_position_impl_trait_in_trait)]
#![feature(test)]

pub mod agent;
pub mod behaviors;
pub mod error;
pub mod ids;
pub mod market;
pub mod prelude;
pub mod state;

use prelude::*;
use rayon::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Opts {
    pub ticks: u32,
}

#[derive(Debug, Clone)]
pub struct TickOutput {
    pub ctx: Context,
    pub actions: Vec<(AgentId, Action)>,
    pub events: Vec<Event>,
}

pub fn run_tick(mut ctx: Context) -> Result<TickOutput> {
    let actions = fetch_agent_actions(&ctx)?;

    // apply agent actions
    let events = apply_actions(&mut ctx, &actions)?;

    // non-agent world processes
    update_world_systems(&mut ctx);

    Ok(TickOutput {
        ctx,
        actions,
        events,
    })
}

pub fn fetch_agent_actions(ctx: &Context) -> Result<Vec<(AgentId, Action)>> {
    // todo: pluggable action api

    // todo: figure out why the fuck this isn't faster?!?!
    // let v = ctx.state.agents.iter().collect::<Vec<_>>();
    // Ok(v.par_iter().map(|(agent_id, agent)| {
    //     let action = agent.act(ctx).unwrap();
    //     dbg!(agent_id);
    //     (**agent_id, action)
    // }).collect::<Vec<_>>())

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

    const UPKEEP: Money = Money(0.1);

    match action {
        Action::Noop => {
            agents = agents.try_update_with(agent_id, |agent| {
                // Upkeep cost for doing nothing
                agent.coins -= UPKEEP;
                Ok(())
            })?;
        }
        Action::Move { port_id } => {
            agents = agents.try_update_with(agent_id, |agent| {
                ensure!(
                    ctx.static_info.are_neighbors(agent.pos, *port_id),
                    "Cannot move to a non-adjacent port"
                );
                agent.pos = *port_id;
                agent.coins -= UPKEEP;
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
            let Some(cost) = port
                .market
                .buy(good, &mut agent.coins, amt) else {
                    warn!("Tried to buy when impossible!");
                    return Ok(ctx.state.clone())
                };
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
            agent.coins -= UPKEEP;

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
            let Some(cost) = port
                .market
                .sell(good, &mut agent.coins, -amt) else {
                    warn!("Tried to sell when impossible!");
                    return Ok(ctx.state.clone())
                };

            agent.cargo = None;
            events.push(Event::Trade {
                port: port.id,
                agent: agent.id,
                good: *good,
                amt,
                cost,
            });
            agent.coins -= UPKEEP;

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
    ctx.state.tick += 1;
}
