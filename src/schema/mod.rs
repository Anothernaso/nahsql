use crate::{
    schema::table::{SchemaTable, SchemaTableBuildError, SchemaTableBuilder},
    value::ValueType,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

pub mod field;
pub mod table;

/// Something is wrong with the scheme.
#[derive(Debug, Error)]
pub enum SchemaBuildError {
    #[error("failed to build child: {0}")]
    ChildBuildError(anyhow::Error),

    #[error("invalid table: {0}, reason: {1}")]
    TableError(String, SchemaBuildTableError),
}

/// Something is wrong with a table of the scheme.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Error)]
pub enum SchemaBuildTableError {
    #[error("invalid field: {0}, reason: {1}")]
    FieldError(String, SchemaBuildFieldError),

    #[error("invalid primary key: {0}, reason: {1}")]
    PrimaryKeyError(String, SchemaBuildPrimaryKeyError),
}

/// Something is wrong with a field of a table.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Error)]
pub enum SchemaBuildFieldError {
    #[error("key field has non-indexable type: {0}")]
    InvalidType(ValueType),
}

/// Something is wrong with the primary key of a table.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Error)]
pub enum SchemaBuildPrimaryKeyError {
    #[error("primary key field not a key")]
    InvalidFieldType,

    #[error("primary key has no respective field")]
    MissingField,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema {
    tables: HashMap<String, SchemaTable>,
    version: usize,
}

impl Schema {
    pub fn tables(&self) -> &HashMap<String, SchemaTable> {
        &self.tables
    }

    pub fn version(&self) -> usize {
        self.version
    }
}

pub struct SchemaBuilder {
    tables: Vec<SchemaTable>,
    version: usize,
}

impl SchemaBuilder {
    pub fn new(
        setup: impl FnOnce(Self) -> anyhow::Result<Self>,
    ) -> Result<Schema, SchemaBuildError> {
        setup(Self {
            tables: vec![],
            version: 0,
        })
        .map_err(|e| SchemaBuildError::ChildBuildError(e))?
        .build()
    }

    /// Manually adds a table to the schema.
    ///
    /// # Notes
    ///
    /// It is recommended that you use `Self::table` instead.
    ///
    pub fn add_table(&mut self, table: SchemaTable) {
        self.tables.push(table);
    }

    /// Adds a new table to the schema,
    /// where `name` is the name of the table
    /// and `setup` is a closure for setting up the table.
    ///
    /// It returns a table builder,
    /// temporarily consuming the schema builder until the table has been built.
    ///
    pub fn table(
        self,
        name: impl Into<String>,
        setup: impl FnOnce(SchemaTableBuilder) -> anyhow::Result<SchemaTableBuilder>,
    ) -> Result<Self, SchemaTableBuildError> {
        SchemaTableBuilder::new(self, name.into(), setup)
    }

    /// Sets the version of the schema.
    ///
    /// The version can be used to identify
    /// which version of the schema was
    /// used when creating a database.
    ///
    pub fn version(mut self, version: usize) -> Self {
        self.version = version;

        self
    }

    /// Builds the schema as configured, consuming the builder.
    fn build(self) -> Result<Schema, SchemaBuildError> {
        let mut tables = HashMap::<String, SchemaTable>::new();

        // Verify all the tables
        self.tables.into_iter().try_for_each(|t| {
            let name = t.name();
            let primary_key = t.primary_key();
            let fields = t.fields();

            // Verify that all fields marked as keys are indexable
            fields
                .iter()
                .find(|(_, f)| f.is_key() && !f.r#type().is_indexable())
                .map_or(Ok(()), |(_, f)| {
                    Err(SchemaBuildError::TableError(
                        name.to_owned(),
                        SchemaBuildTableError::FieldError(
                            f.name().to_owned(),
                            SchemaBuildFieldError::InvalidType(f.r#type()),
                        ),
                    ))
                })?;

            // Verify that the table's primary key is a valid field in the table.
            let primary_key_field = fields.get(primary_key).ok_or_else(|| {
                SchemaBuildError::TableError(
                    name.to_owned(),
                    SchemaBuildTableError::PrimaryKeyError(
                        primary_key.to_owned(),
                        SchemaBuildPrimaryKeyError::MissingField,
                    ),
                )
            })?;

            // Verify that the table's primary key is marked as a key.
            if !primary_key_field.is_key() {
                return Err(SchemaBuildError::TableError(
                    name.to_owned(),
                    SchemaBuildTableError::PrimaryKeyError(
                        primary_key.to_owned(),
                        SchemaBuildPrimaryKeyError::InvalidFieldType,
                    ),
                ));
            }

            tables.insert(t.name().to_owned(), t);

            Ok(())
        })?;

        Ok(Schema {
            tables,
            version: self.version,
        })
    }
}
