use crate::prelude::*;

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
)]
#[serde(transparent)]
pub struct Good {
    pub name: Ustr,
}
