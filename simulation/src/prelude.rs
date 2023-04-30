// dependencies
pub use color_eyre::eyre::{eyre, Result};
pub use derive_more::{
    Add, Deref, DerefMut, Display, Div, From, Into, Mul, Sub,
};
pub use log::{debug, error, info, warn};
pub use petgraph::graph::{NodeIndex, UnGraph as Graph};
pub use rpds::{HashTrieMap as HTMap, HashTrieSet as HTSet, Vector};
pub use serde::{Deserialize, Serialize};
pub use std::{default::Default, error::Error, rc::Rc};
use std::{hash::Hash, ops::Index};
pub use ustr::{ustr, Ustr};
use petgraph::prelude::*;

// crate's modules
pub use crate::{agent::*, ids::*, market::*, state::*};

pub trait Update<K, V> {
    fn update(&self, key: K, val: V) -> Self;

    fn update_with(&self, key: K, f: impl FnOnce(&mut V)) -> Self;
}

impl<K: Hash + Eq, V: Clone> Update<K, V> for HTMap<K, V> {
    fn update(&self, key: K, val: V) -> Self {
        self.remove(&key).insert(key, val)
    }

    fn update_with(&self, key: K, f: impl FnOnce(&mut V)) -> Self {
        let mut v = self.index(&key).clone();
        f(&mut v);
        self.update(key, v)
    }
}
