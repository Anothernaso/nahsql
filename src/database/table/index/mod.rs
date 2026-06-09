use crate::value::ValueKey;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Display, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[display("DbTableIndex {{ entries: {:?} }}", entries)]
pub struct DbTableIndex {
    /// The structure is `<key_field_value, entry_primary_key>`
    entries: HashMap<ValueKey, ValueKey>,
}

impl DbTableIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entries(&self) -> &HashMap<ValueKey, ValueKey> {
        &self.entries
    }

    pub fn entries_mut(&mut self) -> &mut HashMap<ValueKey, ValueKey> {
        &mut self.entries
    }
}
