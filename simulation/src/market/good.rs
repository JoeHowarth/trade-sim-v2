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
