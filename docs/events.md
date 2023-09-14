# Events

Saving the complete state per tick is helpful, but not sufficient to 
understand everything that occurred. 
The motivating example is a trade being processed. This can be backed 
out by looking at the action taken by an agent and the before and after 
state, but it's easier and less ambiguous to have a record showing what 
happened explicitly. 

This concept can be further generalized to other types of interactions later in the simulation. 
Possible examples:
- agent dies 
- trade occurs 
- upgrade completes  

## Implementation

- `History` gets new `events` field
- `Event` enum 
