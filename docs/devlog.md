# Devlog
## Sunday 9/24/23
- Moved over to pixijs 2D renderer
- Made simple scene with rectangles + interactions
- Extended with simple graph creation system 
- Now unsure how to proceed
- Want: 
  - Scaffold for good workflow 

## Saturday 9/23/23
[10:00 AM] Today we have octoberfest at 3pm, so only ~4 hours of work 
- Need a behavior that treats routes as first class
- Would like stateful behaviors: make plan -> execute plan
- Want to start on Visualization 
  - Web based, react + konva 

Retro:
- Set up vite + mantine styles + storybook
- Experimented with cytoscape js (network viewer library)
- Cytoscape not flexible enough to show agents etc. 

## Friday 9/22/23
I want to move on to an actual game "Epic of Emporia" (see notion), 
but I want to have something complete here first.
What's left:
- Non-linear exchanger
- A visualization
- Hosting 
- Multi-good 

Today I want to make a non-linear exchanger based off a/x + b instead of -mx + c
Retro:
- Spent a long time debugging exhaustive behavior
  - Unstable and often prioritizes noops so that the terminal state ends with holding a good or selling it instead of buying
  - Hard to visualize all action branches 
  - Slow 
  - No discounting of future rewards
- Finished non-linear pricer easily and quickly, but not super impactful yet  

## Friday 9/15/23
Goals:
- Today I want to build a look-ahead agent behavior that can get 
  most topologies into equilibrium with enough agents
- If this works, next I want to start a visualization in bevy

Log
- [11:00] Feeling a bit tired today and have the blue origins tour at 4

## Wed 9/13/23 (retroactive)
- Did a bunch of analysis work and learned polars much better
- Created 'routes' table
- Added events
- Refactored logic into free functions instead of obj methods

## Tue 9/12/23

- I want to have a "thing" by the end of this week's work-break
- Want to introduce branch discipline
- Want to figure out how to use a debugger w/ rust
- Want a crash report file that can be loaded up so crash can be reproduced (and looked at with debugger!)
- By eod want a good debugging workflow for applying actions
- Note: would like to refactor logic out of obj methods and into free
  functions that operate on objects
- 5:50 ~5hrs later. Today was a good day. I:
  - Figured out how to use a debugger with rust
  - Overhauled the error system in `apply_action` to be more ergonomic
  - Created a useful CrashReport system that's easy to work with
  - Added CI to ensure unit tests keep passing
  - Fixed a bug + expanded unit testing in the `Exchanger` + `Pricer`
- 7:38
  - Plots for agent locations and prices_by_port
  - Noticed agents didn't seem to be changing pricing of ports... problem!!
  - Added a bunch of unit tests for agent behavior and found several bugs
  - Another win for interpretability!!!

## Wed 9/6/23

- Took off work early, want to spend afternoon making debugging great
- Possibilities: When there's an error: (OR)
  - history obj has error saved on it
  - python error contains full state
  - write history to file

## 2023 8/31/2023 Thursday

- Reorienting myself with the codebase
  - Ran into `Invalid Action: tried to sell when impossible`
    - Hard to debug
    - What was the situation when this occurred?
    - Can we capture more information to make debugging easier?

## 2023-05-05 Saturday

- [ ] basic agent behavior
- [ ] update markets per tick
- [ ] export tables using polars
- [ ] notebooks folder w/ loading and parsing output data
