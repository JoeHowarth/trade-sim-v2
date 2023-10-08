# Streaming Interface

**Goal: Mimic a rust channel, but over stdin/stdout**

Problems

- Serialization format needs to be readable across languages, ideally without a spec
- Framing: When does a message start and stop

## Approach 1 - Use JSON and read_line

- Simple
- Language agnostic
- Low performance
- Can't be improved if server re-written in rust

## Approach 2 - Use length + binary data
- Requires working with `Read` trait directly
- Possibly more efficient, but still need to choose a serde format for data payload itself (likely json anyway)
- Easy to change data format later


## Discussion
- Start with 1
- Move to 2 if we experience perf issues 
