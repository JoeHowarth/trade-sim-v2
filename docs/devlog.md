# Devlog

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
