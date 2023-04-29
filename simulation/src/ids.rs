use crate::prelude::*;

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
pub struct PortId(pub Ustr);

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
