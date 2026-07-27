use crate::variant::KeyVariant;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Display, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[display(
    r#"
Table Index:
    Entries - {:?}
"#,
    entries
)]
pub struct TbIndex {
    /// The structure is `(key_field_value, entry_primary_key)`
    entries: HashSet<(KeyVariant, KeyVariant)>,
}

impl TbIndex {
    pub fn new(entries: impl Into<HashSet<(KeyVariant, KeyVariant)>>) -> Self {
        Self {
            entries: entries.into(),
        }
    }

    pub fn entries(&self) -> &HashSet<(KeyVariant, KeyVariant)> {
        &self.entries
    }

    pub fn entries_mut(&mut self) -> &mut HashSet<(KeyVariant, KeyVariant)> {
        &mut self.entries
    }

    pub fn get_entries(self) -> HashSet<(KeyVariant, KeyVariant)> {
        self.entries
    }

    pub fn set_entries(&mut self, normal: impl Into<HashSet<(KeyVariant, KeyVariant)>>) {
        self.entries = normal.into();
    }
}

impl AsRef<Self> for TbIndex {
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl From<&TbIndex> for TbIndex {
    fn from(value: &TbIndex) -> Self {
        value.clone()
    }
}
