# Devlog

## Thur 9/28/23
[10:00 AM]
I'm reducing the scope of this project!  It will only be a tech demo, 
that shows the full architecture the real game will use. 

To that end, the components I need are:
- Playback of history file
- Output and input streaming for Simulation cli
- Server streaming of current tick 
- Hosting 
- UI page for starting, saving, loading etc. a sim run  
- Rewrite the sim logic to use Bevy ECS (or at least evaluate)
- Route-based agent behavior (not strictly needed, but come on...)

Up first:
- Fix rendering agents
- Playback


## Wed 9/27/23
[11:47 AM] Late start, made soup and read at bequest cafe
- Haven't been getting deep sleep and having some motivation issues 
- I like the architecture currently, but I'm worried there's too much focus on 
  the UI and server glue compared to actually simulating things.
- I also don't have a great idea what the UI should look like - mostly making it up 
  as I go along which feels bad.
- Might try working with figma a bit to mock out UI? Maybe that's a waste of time...

What do I want from a UI?
- Control what is displayed on nodes
  - Choose map type
  - Basic list of map types works, but what if they become too many like in EU4?
  - Optional args
    - Pop up?
      - What about cycling between inputs? opening a pop up becomes irritating.
      - Closes on submit by default, but have a toggle that keeps it open on submit?
    - Inputs inline with map mode selector
    - Could look funny..
  - Save map modes w/ args for quick selection? 
    - Could encode mapmode into url?
- Display for agents too, but less straightforward
- Playback: pause, play, +/- speed, jump to tick
- Start run
  - Select existing input files
  - Change some settings 
  - [Stretch] create whole input from scratch 

Todo:
- [] Include agents in visualization
  - [X] Agents endpoint
  - [] Render agents 
- [] Make it play through whole history file


## Tue 9/26/23
[8:44 AM] Today I want to build a "map mode" style network visualization system 
- Nodes can visualize data based off:
  - Color
  - Size
  - Label 
- Agents represented by markers with labels. Same data viz options ^
- UI container holding map mode buttons 
- Hotkeys for selecting map modes
- Hover tooltips (likely react, but maybe pixi text?)

Log
- Played around with rspc (rust trpc). Decent support for exposing methods from rust to typescript simply
- Thought through sim <-> server <-> app flow and decided what the server is written in doesn't really matter
- Sticking with FastAPI for now 
- [1:50 pm] Moving on to map mode work 

Retro
- Got map modes working for markets quite nicely 
- Can color nodes easily 
- Automatic bounds finding (max, min) to scale color space 
- Reorged nodes into a `NetworkNode` class with a border, body, label and label background


## Sunday 9/24/23
- Moved over to pixijs 2D renderer
- Made simple scene with rectangles + interactions
- Extended with simple graph creation system 
- Now unsure how to proceed
- Want: 
  - Scaffold for good workflow 

Retro:
- Improved workflow a lot
- Refactored python logic into files, cleaned up notebook
- Invoke cli instead of using PyO3 bindings 
  - Don't have to maintain python bindings (not too significant)
  - Much simpler dev workflow: no `./maturin.sh` watch script, no restarting the notebook to pick up changes, just works
  - Move to file oriented deving: everything is saved
- Create FastApi server 
  - starting with sending static graph information
- Setup OpenAPI TS client generation 
- Hook up network visualization with FastAPI server: can render graphs from output files 

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
