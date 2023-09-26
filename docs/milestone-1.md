# Milestone 1

I tend to never polish projects up and actually release them.
I'd love to send someone a link and say

> "Hey check out this demo simulation I made,
> here are my ideas for the bigger one I'm working on"

It won't be perfect or "Done", but what's a good stopping point to officially start working towards
_Epic of Emporia_?

## Criteria

- Hosted and a user can construct and run a scenario from the app
- App uses graphics to show many types of node and agent based data from the run
- App incorporates the main charts I use in python notebooks while devving
- Supports multiple goods
- Extremely simple city economics

### Example of extremely simple city economics

- "Pops" consume 10 food per tick
- There are multiple types of food, most cities produce 1-2 types
- Cities have fixed yields per food type each Pop can produce
- Pops need to eat a varied diet
  - Eating <10 unit of food => Die
  - Eating 10/1 of 1 food type => X% of dying
  - Eating 10/2 of 2 food types => no effect
  - Eating 10/3 of 3 food types => Y% of reproducing
  - etc.
- Each Pop produces a certain "labor budget" each tick that the city allocates to
  producing food units
- City aims to supply enough food for all Pops first, then progressively focuses of achieving better diets
-

Foods F1, F2

City A
100 Pops
Cat   | F1 | F2
Yield | 15 | 5


City B
