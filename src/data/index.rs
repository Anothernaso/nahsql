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
    pub normal: HashSet<(ValueKey, ValueKey)>,

    /// The structure is `<key_field_value, entry_primary_key>`
    pub unique: HashMap<ValueKey, ValueKey>,
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
