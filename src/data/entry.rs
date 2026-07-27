use crate::variant::Variant;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents an entry of a table in a database.
///
/// # Notes
///
/// This is meant to be used for serialization.
///
#[derive(Debug, Display, Default, Clone, Serialize, Deserialize)]
#[display(
    r#"
Table Entry:
    Fields - {:?}
"#,
    fields
)]
pub struct TbEntry {
    /// The structure is `<field_name, field_value>`
    fields: HashMap<String, Variant>,
}

impl TbEntry {
    pub fn new(fields: impl Into<HashMap<String, Variant>>) -> Self {
        Self {
            fields: fields.into(),
        }
    }

    pub fn fields(&self) -> &HashMap<String, Variant> {
        &self.fields
    }

    pub fn fields_mut(&mut self) -> &mut HashMap<String, Variant> {
        &mut self.fields
    }

    pub fn get_fields(self) -> HashMap<String, Variant> {
        self.fields
    }

    pub fn set_fields(&mut self, fields: impl Into<HashMap<String, Variant>>) {
        self.fields = fields.into();
    }
}

impl AsRef<Self> for TbEntry {
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl From<&TbEntry> for TbEntry {
    fn from(value: &TbEntry) -> Self {
        value.clone()
    }
}
