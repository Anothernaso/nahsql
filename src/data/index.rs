use crate::value::ValueKey;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Display, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[display(
    r#"
Table Index:
    Entries - {:?}
    "#,
    entries
)]
pub struct DbIndex {
    /// The structure is `<key_field_value, entry_primary_key>`
    pub entries: HashMap<ValueKey, ValueKey>,
}

impl AsRef<DbIndex> for DbIndex {
    fn as_ref(&self) -> &DbIndex {
        &self
    }
}
