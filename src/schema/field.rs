use crate::{schema::table::SchemaTableBuilder, value::ValueType};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SchemaTableFieldBuildError {
    #[error("failed to build child: {0}")]
    ChildBuildError(anyhow::Error),

    #[error("missing type")]
    MissingType,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SchemaTableField {
    name: String,

    r#type: ValueType,
    is_key: bool,
}

impl SchemaTableField {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn r#type(&self) -> ValueType {
        self.r#type
    }

    pub fn is_key(&self) -> bool {
        self.is_key
    }
}

pub struct SchemaTableFieldBuilder {
    builder: SchemaTableBuilder,

    name: String,
    r#type: Option<ValueType>,
    is_key: bool,
}

impl SchemaTableFieldBuilder {
    pub fn new(
        builder: SchemaTableBuilder,
        name: impl Into<String>,
        setup: impl FnOnce(SchemaTableFieldBuilder) -> anyhow::Result<SchemaTableFieldBuilder>,
    ) -> Result<SchemaTableBuilder, SchemaTableFieldBuildError> {
        setup(Self {
            builder,

            name: name.into(),
            r#type: None,
            is_key: false,
        })
        .map_err(|e| SchemaTableFieldBuildError::ChildBuildError(e))?
        .build()
    }

    /// Sets the primitive type of this field.
    pub fn r#type(mut self, r#type: ValueType) -> Self {
        self.r#type = Some(r#type);

        self
    }

    /// Marks the field as a key that should be indexed.
    pub fn key(mut self) -> Self {
        self.is_key = true;

        self
    }

    /// Builds this field, consuming the field builder and returns the underlying table builder.
    fn build(mut self) -> Result<SchemaTableBuilder, SchemaTableFieldBuildError> {
        self.builder.add_field(SchemaTableField {
            name: self.name,
            r#type: self
                .r#type
                .ok_or_else(|| SchemaTableFieldBuildError::MissingType)?,
            is_key: self.is_key,
        });

        Ok(self.builder)
    }
}
