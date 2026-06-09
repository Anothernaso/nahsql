use crate::value::Value;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Display, Default, Clone, PartialEq, Serialize, Deserialize)]
#[display("DbTableEntryManifest {{ fields: {:?} }}", fields)]
pub struct DbTableEntryManifest {
    /// The structure is `<field_name, field_value>`
    fields: HashMap<String, Value>,
}

impl DbTableEntryManifest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fields(&self) -> &HashMap<String, Value> {
        &self.fields
    }

    pub fn fields_mut(&mut self) -> &mut HashMap<String, Value> {
        &mut self.fields
    }
}
