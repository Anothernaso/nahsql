use crate::value::Value;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents an entry of a table in a database.
///
/// # Notes
///
/// This is meant to be used for serialization.
///
#[derive(Debug, Display, Default, Clone, PartialEq, Serialize, Deserialize)]
#[display(
    r#"
Table Entry:
    Fields - {:?}
    "#,
    fields
)]
pub struct Entry {
    /// The structure is `<field_name, field_value>`
    fields: HashMap<String, Value>,
}

impl Entry {
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
