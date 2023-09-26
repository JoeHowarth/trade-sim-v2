# How to serve data to the web viewer

### Use cases

- Serving prevous output file
- Serving running, but non-interactive simulation
- Serving running simulation, with agent interactions and tick speed control
- Reuse analysis code for python notebook exploration

## Simulation Facing API

Use stdout / stdin for everything
Invent a simple message frame system

- After each tick, serialize all state and push it out through stdout
- For all agents that have input behaviors, wait to receive input from stdin
  - Most behaviors don't need outside input
  - Those that do, likely higher level "plans" or "goals" that are interpreted
    by sim-side code.
  - Each behavior reponds with enum to sim: Action | BlockOnInput
- For sim level commands (e.g. pause, target ms per frame, etc.) check for them before start of tick

### FastAPI

- python server
- invokes rust cli with args
-

### Rust approach
