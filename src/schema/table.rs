use crate::schema::{
    SchemaBuilder,
    field::{SchemaTableField, SchemaTableFieldBuildError, SchemaTableFieldBuilder},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SchemaTableBuildError {
    #[error("failed to build child: {0}")]
    ChildBuildError(anyhow::Error),

    #[error("missing primary key")]
    MissingPrimaryKey,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaTable {
    name: String,

    fields: HashMap<String, SchemaTableField>,
    primary_key: String,
}

impl SchemaTable {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fields(&self) -> &HashMap<String, SchemaTableField> {
        &self.fields
    }

    pub fn primary_key(&self) -> &str {
        &self.primary_key
    }
}

pub struct SchemaTableBuilder {
    builder: SchemaBuilder,
    name: String,

    fields: Vec<SchemaTableField>,
    primary_key: Option<String>,
}

impl SchemaTableBuilder {
    pub fn new(
        builder: SchemaBuilder,
        name: impl Into<String>,
        setup: impl FnOnce(Self) -> anyhow::Result<Self>,
    ) -> Result<SchemaBuilder, SchemaTableBuildError> {
        setup(Self {
            builder,
            name: name.into(),

            fields: vec![],
            primary_key: None,
        })
        .map_err(|e| SchemaTableBuildError::ChildBuildError(e))?
        .build()
    }

    /// Manually adds a field to the table.
    ///
    /// # Notes
    ///
    /// It is recommended that you use `Self::field` instead.
    ///
    pub fn add_field(&mut self, field: SchemaTableField) {
        self.fields.push(field);
    }

    /// Adds a new field to the table,
    /// where `name` is the name of the field
    /// and `setup` is a closure for setting up the field.
    ///
    /// It returns a field builder,
    /// temporarily consuming the table builder until the field has been built.
    ///
    pub fn field(
        self,
        name: impl Into<String>,
        setup: impl FnOnce(SchemaTableFieldBuilder) -> anyhow::Result<SchemaTableFieldBuilder>,
    ) -> Result<SchemaTableBuilder, SchemaTableFieldBuildError> {
        SchemaTableFieldBuilder::new(self, name, setup)
    }

    /// Sets the name of the field to use as the primary key of the table.
    pub fn primary_key<S: Into<String>>(mut self, primary_key: S) -> Self {
        self.primary_key = Some(primary_key.into());

        self
    }

    /// Builds this table, consuming the table builder and returns the underlying schema builder.
    fn build(mut self) -> Result<SchemaBuilder, SchemaTableBuildError> {
        self.builder.add_table(SchemaTable {
            name: self.name,
            fields: self
                .fields
                .into_iter()
                .map(|field| (field.name().to_owned(), field))
                .collect(),
            primary_key: self
                .primary_key
                .ok_or_else(|| SchemaTableBuildError::MissingPrimaryKey)?,
        });
        Ok(self.builder)
    }
}
