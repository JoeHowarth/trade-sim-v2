# Replay Crash from Dumped State

### User experience

- Run simulation normally from either ui, notebook or cli
- Hit an error in simulation code (applying actions or background systems)
- Record
  - current state right before crash
  - error message
  - line number
  - stack trace?
- Recorded state should enable a repro of the crash. Likely need:
  - Sim input
  - History (not including current tick)
  - State right before crash
  - Actions already applied
  - Actions left to apply
- Entry point that takes in this state and runs forward 1 tick (to repro crash)

### Implementation

Types:

- Simulation Error: ApplyActionError or SystemUpdateError

In error case:

- `Context`
  - Break apart getting agent actions from applying actions and updating world systems
  - If error occurs:
    - collect state from current place in loop
    - partition actions into applied and not yet applied
- `History`
  - Downcasts to SimulationError and enriches with previous history
  - Writes error information to file
  - Returns error to caller

File format?

### Commits:

1. Create crash report type
2. Define crash report file format and save it to disk
3. Entrypoint to load crash report and advance 1 tick
4. Expose entrypoint from cli and test with debugger
5. Write jupyter notebook / python utils to load and analyze crash report
