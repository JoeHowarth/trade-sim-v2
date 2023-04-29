#![allow(unused_imports, dead_code, incomplete_features)]
#![feature(return_position_impl_trait_in_trait)]

pub mod market;
pub mod prelude;

use market::Market;
use prelude::*;

#[derive(Debug, Clone)]
struct Port {
    pub price: f32,
    pub market: Market,
}

#[derive(Debug, Clone, Default)]
struct GraphNode {
    name: Ustr,
    graph_idx: NodeIndex,
}

#[derive(Debug, Clone, Default)]
struct GraphEdge {}

#[derive(Debug, Clone, Default)]
struct Agent {}

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    Deref,
    DerefMut,
    From,
    PartialEq,
    Eq,
    Hash,
)]
struct PortId(pub Ustr);

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    Deref,
    DerefMut,
    From,
    PartialEq,
    Eq,
    Hash,
)]
struct AgentId(pub Ustr);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
