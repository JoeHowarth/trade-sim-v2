use std::collections::{HashSet, VecDeque};

use crate::{prelude::*, apply_action};

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

    // info!("Apply {}", &action);
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
    path: rpds::List<Action>,
    depth: u8,
) -> Result<(SearchState, rpds::List<Action>)> {
    if depth == 0 {
        // info!("");
        // info!("Terminal");
        // info!(
        //     "Final state: {} {:?}     {}",
        //     sstate.agent.pos, sstate.agent.cargo, sstate.agent.coins
        // );
        // let mut s = String::with_capacity(100);
        // s.push_str("[");
        // for x in &path {
        //     s.push_str(&format!("{}, ", x));
        // }
        // s.push_str("]");
        // info!("{}", s);
        // info!("Evaluate: {}", evaluate(ctx, &sstate));
        return Ok((sstate, path));
    }

    let mut max_value = i32::MIN;
    let mut ret = None;

    for action in valid_actions(ctx, &sstate) {
        let new_state = simulate(ctx, &sstate, &action)?;
        let (terminal_state, actions) = dfs(
            ctx,
            new_state,
            visited,
            path.push_front(action.clone()),
            depth - 1,
        )?;

        let val = evaluate(&ctx, &terminal_state);
        if val >= max_value {
            max_value = val;
            ret = Some((terminal_state, actions));
        }
    }

    Ok(ret.unwrap_or((sstate, path)))
}

pub fn act_exhaustive(agent: &Agent, ctx: &Context, depth: u8) -> Result<Action> {
    let sstate = SearchState {
        tick: ctx.state.tick,
        agent: agent.clone(),
    };
    let mut visited = HashSet::with_capacity(100);

    let (best_state, actions) = dfs(ctx, sstate, &mut visited, rpds::List::new(), depth)?;

    let mut reversed = actions.iter().cloned().collect::<Vec<_>>();
    reversed.reverse();
    info!("True Best actions:     {:?}", &reversed);
    info!("True Final state:     {:?}", &best_state);
    info!("True Final evaluation {:?}", evaluate(ctx, &best_state));
    assert_eq!(reversed.get(0).unwrap(), &actions.last().unwrap().clone());

    actions
        .last()
        .cloned()
        .ok_or_else(|| eyre!("Expected best path to have at least one action"))
}

fn evaluate(ctx: &Context, sstate: &SearchState) -> i32 {
    let value_if_sold = if let Some(good) = sstate.agent.cargo {
        1.0 * -ctx
            .state
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

    // #[test]
    // fn fuckme() {
    //     let mut x = rpds::List::new();
    //     x = x.push_front("hi");
    // }

    #[test]
    fn valid_actions_test() {
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

        let sstate_1 = simulate(
            &ctx,
            &sstate,
            &Action::BuyAndMove {
                good: "Good".into(),
                port_id: high_port().id,
            },
        )
        .unwrap();

        assert_eq!(
            &sstate_1,
            &SearchState {
                tick: 2,
                agent: Agent {
                    pos: "high_port".into(),
                    cargo: Some("Good".into()),
                    coins: (448.5).into(),
                    ..sstate.agent.clone()
                }
            }
        );

        let actions: Vec<Action> = valid_actions(&ctx, &sstate_1).collect();
        assert_eq!(
            &actions,
            &[
                Action::Noop,
                Action::Move {
                    port_id: "mid_port".into()
                },
                Action::Move {
                    port_id: "low_port".into()
                },
                Action::Sell {
                    good: "Good".into()
                }
            ]
        );

        let (terminal, actions) =
            dfs(&ctx, sstate_1, &mut HashSet::new(), Default::default(), 1).unwrap();
        let actions = Vec::from_iter(actions.iter().cloned());

        assert_eq!(
            actions.last().unwrap(),
            &Action::Sell {
                good: "Good".into()
            }
        );
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
    fn two_step_dfs() {
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

        let (best_state, actions) = dfs(&ctx, sstate, &mut visited, rpds::List::new(), 2).unwrap();

        let mut actions = actions.into_iter().cloned().collect::<Vec<_>>();
        actions.reverse();

        dbg!(&actions);
        assert_eq!(
            actions.as_slice(),
            &[
                Action::Move {
                    port_id: "low_port".into()
                },
                Action::BuyAndMove {
                    good: "Good".into(),
                    port_id: high_port().id
                },
            ]
        );
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
            let sstate = SearchState {
                tick: ctx.state.tick,
                agent: agent.clone(),
            };
            let mut visited = HashSet::with_capacity(100);

            let (best_state, actions) =
                dfs(&ctx, sstate, &mut visited, rpds::List::new(), 6).unwrap();

            let mut actions = actions.into_iter().cloned().collect::<Vec<_>>();
            actions.reverse();
            dbg!(&actions);

            let low = low_port().id;
            let high = high_port().id;
            // dbg!(&action);
            let good = "Good".into();
            assert_eq!(
                &actions,
                &[
                    Action::Move { port_id: low },
                    Action::BuyAndMove {
                        good,
                        port_id: high,
                    },
                    Action::Sell { good },
                    Action::Move { port_id: low },
                    Action::BuyAndMove {
                        good,
                        port_id: high,
                    },
                    Action::Sell { good },
                ]
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
            let before = agent.coins;
            let received = -ctx
                .state
                .ports
                .g(sstate.agent.pos)
                .market
                .clone()
                .sell(&good, &mut agent.coins, 1)
                .unwrap();
            println!("{:?}", received);
            assert_eq!(eval, (before + (received * 1.0)).0.round() as i32);
        }
    }
}
