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
pub struct TbEntry {
    /// The structure is `<field_name, field_value>`
    pub fields: HashMap<String, Value>,
}

impl AsRef<Self> for TbEntry {
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl Into<TbEntry> for &TbEntry {
    fn into(self) -> TbEntry {
        self.clone()
    }
}
