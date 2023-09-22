## Look Ahead Agent AI

## Features
- The agent 'looks' N steps ahead to choose a goal
- The agent can reason at multiple levels of abstraction
    - Concretly, the agent can plan in terms of movement and routes, not just individual per-step actions
    - These high level goals are translated down to the level of the simulation
    - **Note:** this opens up the possibility of async goal finding and sync goal interpretation needed 
      for more complex agents
- [Stretch] formalize into tree-search with pruning

## Approaches

### Exhaustive action space exploration

- Simulation function, approximation
    - (s,a) -> s'
- Enumerate all valid actions from a given state s
- Breadth first search up to a given depth  
- Choose sequence of actions that lead to highest reward

### Higher level action space

- Use `Buy(port)` and `Sell(port)` as action space 
- Once solved, lower to simulation action space 