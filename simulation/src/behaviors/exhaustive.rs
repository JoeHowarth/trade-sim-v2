use std::collections::{HashSet, VecDeque};

use crate::{apply_action, prelude::*};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
struct SearchState {
    tick: u32,
    agent: Agent,
}

fn simulate(ctx: &Context, sstate: &SearchState, action: &Action) -> Result<SearchState> {
    let mut ctx = ctx.clone();
    ctx.state.agents = ctx
        .state
        .agents
        .insert(sstate.agent.id, sstate.agent.clone());

    let state = apply_action(&ctx, action, sstate.agent.id, &mut vec![])?;

    Ok(SearchState {
        tick: sstate.tick + 1,
        agent: state.agents.g(sstate.agent.id).clone(),
    })
}

fn valid_actions(ctx: &Context, sstate: &SearchState) -> impl Iterator<Item = Action> {
    let agent = &sstate.agent;
    let mut actions = Vec::with_capacity(20);
    actions.push(Action::Noop);

    // Can move to neighbors
    for nbr in ctx.static_info.graph.neighbors(agent.pos) {
        actions.push(Action::Move { port_id: nbr });
    }

    if let Some(good) = agent.cargo {
        // Can Sell if have cargo
        actions.push(Action::Sell { good });
    } else {
        let cur_port = ctx.state.ports.g(agent.pos);
        let goods = || cur_port.market.goods();

        // Can Buy and Move if doesn't have cargo
        for nbr in ctx.static_info.graph.neighbors(agent.pos) {
            for &good in goods() {
                actions.push(Action::BuyAndMove { good, port_id: nbr });
            }
        }
    }

    actions.into_iter()
}

fn dfs(
    ctx: &Context,
    sstate: SearchState,
    visited: &mut HashSet<SearchState>,
    depth: u8,
) -> Result<(SearchState, VecDeque<Action>)> {
    println!("Frame, depth: {depth}");
    if depth == 0 {
        return Ok((sstate, VecDeque::new()));
    }

    visited.insert(sstate.clone());

    let mut max_value = evaluate(ctx, &sstate);
    let mut ret = None;

    for action in valid_actions(ctx, &sstate) {
        dbg!(&action);
        let new_state = simulate(ctx, &sstate, &action)?;
        if !visited.contains(&new_state) {
            println!("not visited");
            let (terminal_state, mut actions) = dfs(ctx, new_state, visited, depth - 1)?;
            dbg!(&terminal_state, &action, evaluate(ctx, &terminal_state));

            actions.push_front(action);
            let val = evaluate(&ctx, &terminal_state);
            if val > max_value {
                max_value = val;
                ret = Some((terminal_state, actions));
            }
        }
    }

    Ok(ret.unwrap_or_else(|| (sstate, VecDeque::new())))
}

pub fn act_exhaustive(agent: &Agent, ctx: &Context, depth: u8) -> Result<Action> {
    let sstate = SearchState {
        tick: ctx.state.tick,
        agent: agent.clone(),
    };
    let mut visited = HashSet::with_capacity(100);

    let (best_state, mut actions) = dfs(ctx, sstate, &mut visited, depth)?;

    dbg!(&actions);
    dbg!(&best_state);
    dbg!(evaluate(ctx, &best_state));

    actions
        .pop_front()
        .ok_or_else(|| eyre!("Expected best path to have at least one action"))
}

fn evaluate(ctx: &Context, sstate: &SearchState) -> i32 {
    dbg!(&sstate);
    let value_if_sold = if let Some(good) = sstate.agent.cargo {
        -ctx.state
            .ports
            .g(sstate.agent.pos)
            .market
            .cost(&good, -1)
    } else {
        Money::from(0.)
    };
    (value_if_sold + sstate.agent.coins).0.round() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::behaviors::tests::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn one_step_from_mid_with_cargo() -> Result<()> {
        let ctx = ctx();
        let agent = Agent {
            id: "A1".into(),
            pos: mid_port().id,
            cargo: Some("Good".into()),
            coins: (500.).into(),
            behavior: Behavior::Exhaustive,
        };

        let action = act_exhaustive(&agent, &ctx, 1)?;

        dbg!(&action);
        assert_eq!(
            &action,
            &Action::Move {
                port_id: high_port().id
            }
        );
        Ok(())
    }

    #[test]
    fn one_step_from_mid_no_cargo() -> Result<()> {
        let ctx = ctx();
        let agent = Agent {
            id: "A1".into(),
            pos: mid_port().id,
            cargo: None,
            coins: (500.).into(),
            behavior: Behavior::Exhaustive,
        };

        let action = act_exhaustive(&agent, &ctx, 1)?;

        dbg!(&action);
        assert_eq!(
            &action,
            &Action::BuyAndMove {
                good: "Good".into(),
                port_id: high_port().id
            }
        );
        Ok(())
    }

    #[test]
    fn two_step_dfs() -> Result<()> {
        let ctx = ctx();
        let agent = Agent {
            id: "A1".into(),
            pos: mid_port().id,
            cargo: None,
            coins: (500.).into(),
            behavior: Behavior::Exhaustive,
        };

        let sstate = SearchState {
            tick: ctx.state.tick,
            agent: agent.clone(),
        };
        let mut visited = HashSet::with_capacity(100);

        let (best_state, actions) = dfs(&ctx, sstate, &mut visited, 2)?;

        let actions = actions.into_iter().collect::<Vec<_>>();

        dbg!(&actions);
        assert_eq!(
            actions.as_slice(),
            &[
                Action::Move {
                    port_id: low_port().id
                },
                Action::BuyAndMove {
                    good: "Good".into(),
                    port_id: high_port().id
                }
            ]
        );
        Ok(())
    }

    #[bench]
    fn perf(b: &mut Bencher) {
        b.iter(|| {
            let ctx = ctx();
            let agent = Agent {
                id: "A1".into(),
                pos: mid_port().id,
                cargo: None,
                coins: (500.).into(),
                behavior: Behavior::Exhaustive,
            };

            let action = act_exhaustive(&agent, &ctx, 10).unwrap();

            dbg!(&action);
            assert_eq!(
                &action,
                &Action::BuyAndMove {
                    good: "Good".into(),
                    port_id: high_port().id
                }
            );
        });
    }

    #[test]
    fn test_evaluate() {
        let ctx = ctx();

        {
            let agent = Agent {
                cargo: None,
                coins: (500.).into(),
                ..agent()
            };
            let sstate = SearchState { tick: 1, agent };
            assert_eq!(evaluate(&ctx, &sstate), 500);
        }

        {
            let good = "Good".into();
            let mut agent = Agent {
                cargo: Some(good),
                coins: (500.).into(),
                pos: mid_port().id,
                ..agent()
            };
            let sstate = SearchState {
                tick: 1,
                agent: agent.clone(),
            };
            let eval = evaluate(&ctx, &sstate);
            assert!(ctx
                .state
                .ports
                .g(sstate.agent.pos)
                .market
                .clone()
                .sell(&good, &mut agent.coins, 1)
                .is_some());
            assert_eq!(eval, agent.coins.0.round() as i32);
        }
    }
}
