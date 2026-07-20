use crate::value::ValueKey;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Display, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[display(
    r#"
Table Index:
    Entries - {:?}
"#,
    unique
)]
pub struct TbIndex {
    /// The structure is `(key_field_value, entry_primary_key)`
    normal: HashSet<(ValueKey, ValueKey)>,

    /// The structure is `<key_field_value, entry_primary_key>`
    unique: HashMap<ValueKey, ValueKey>,
}

impl TbIndex {
    pub fn new(
        normal: impl Into<HashSet<(ValueKey, ValueKey)>>,
        unique: impl Into<HashMap<ValueKey, ValueKey>>,
    ) -> Self {
        Self {
            normal: normal.into(),
            unique: unique.into(),
        }
    }

    pub fn normal(&self) -> &HashSet<(ValueKey, ValueKey)> {
        &self.normal
    }

    pub fn normal_mut(&mut self) -> &mut HashSet<(ValueKey, ValueKey)> {
        &mut self.normal
    }

    pub fn set_normal(&mut self, normal: impl Into<HashSet<(ValueKey, ValueKey)>>) {
        self.normal = normal.into();
    }

    pub fn unique(&self) -> &HashMap<ValueKey, ValueKey> {
        &self.unique
    }

    pub fn unique_mut(&mut self) -> &mut HashMap<ValueKey, ValueKey> {
        &mut self.unique
    }

    pub fn set_unique(&mut self, unique: impl Into<HashMap<ValueKey, ValueKey>>) {
        self.unique = unique.into();
    }
}

impl AsRef<Self> for TbIndex {
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl Into<TbIndex> for &TbIndex {
    fn into(self) -> TbIndex {
        self.clone()
    }
}
