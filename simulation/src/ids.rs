use crate::prelude::*;

#[derive(
    Debug,
    Serialize,
    Deserialize,
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
pub struct PortId(pub Ustr);

#[derive(
    Serialize,
    Deserialize,
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
pub struct AgentId(pub Ustr);

#[derive(
    Deserialize,
    Serialize,
    Eq,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Hash,
    From,
    Into,
    Display,
)]
#[serde(transparent)]
pub struct Good {
    pub name: Ustr,
}

impl From<&str> for Good {
    fn from(value: &str) -> Self {
        Ustr::from(value).into()
    }
}

impl From<&str> for PortId {
    fn from(value: &str) -> Self {
        Ustr::from(value).into()
    }
}
impl From<&str> for AgentId {
    fn from(value: &str) -> Self {
        Ustr::from(value).into()
    }
}
