use crate::value::ValueKey;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Display, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[display(
    r#"
Table Manifest:
    Entries - {:?}
"#,
    entries
)]
pub struct TbManifest {
    /// List of all primary keys in the table
    pub entries: HashSet<ValueKey>,
}

impl AsRef<Self> for TbManifest {
    fn as_ref(&self) -> &Self {
        &self
    }
}
