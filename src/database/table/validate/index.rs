use crate::{
    database::{
        Database, DbError,
        table::index::{DbTableIndex, DbTableIndexImpl},
    },
    schema::field::SchemaTableField,
};

/// Makes the given table index
/// match the database's schema in the filesystem.
///
/// Returns a `DbError` if anything goes wrong.
///
#[cfg(all(feature = "std"))]
pub fn validate_table_index_sync(
    db: &Database,
    table_name: impl AsRef<str>,
    field: &SchemaTableField,
    pkey_field: &SchemaTableField,
) -> Result<(), DbError> {
    use std::fs;

    let table_name = table_name.as_ref();

    let index_path = db.table_index_path(table_name, field.name());
    let mut index: DbTableIndex;

    // Create the index if it doesn't exist
    if !fs::exists(&index_path).map_err(|e| DbError::IoError(e))? {
        index = DbTableIndex::new();
    }
    // Read the index if it already exists
    else {
        let index_json = fs::read_to_string(&index_path).map_err(|e| DbError::IoError(e))?;

        index = serde_json::from_str(&index_json).map_err(|e| DbError::SerError(e))?;
    }

    // Remove all entries from the index that don't match the types in the schema
    index.entries_mut().retain(|fkey, pkey| {
        fkey.r#type() == field.r#type() && pkey.r#type() == pkey_field.r#type()
    });

    let index_json = serde_json::to_string_pretty(&index).map_err(|e| DbError::SerError(e))?;

    // Write the updated index back to disk
    fs::write(index_path, index_json).map_err(|e| DbError::IoError(e))?;

    Ok(())
}

/// Makes the given table index
/// match the database's schema in the filesystem.
///
/// Returns a `DbError` if anything goes wrong.
///
#[cfg(all(feature = "tokio"))]
pub async fn validate_table_index_async(
    db: &Database,
    table_name: impl AsRef<str>,
    field: &SchemaTableField,
    pkey_field: &SchemaTableField,
) -> Result<(), DbError> {
    use tokio::fs;

    let table_name = table_name.as_ref();

    let index_path = db.table_index_path(table_name, field.name());
    let mut index: DbTableIndex;

    // Create the index if it doesn't exist
    if !fs::try_exists(&index_path)
        .await
        .map_err(|e| DbError::IoError(e))?
    {
        index = DbTableIndex::new();
    }
    // Read the index if it already exists
    else {
        let index_json = fs::read_to_string(&index_path)
            .await
            .map_err(|e| DbError::IoError(e))?;

        index = serde_json::from_str(&index_json).map_err(|e| DbError::SerError(e))?;
    }

    // Remove all entries from the index that don't match the types in the schema
    index.entries_mut().retain(|fkey, pkey| {
        fkey.r#type() == field.r#type() && pkey.r#type() == pkey_field.r#type()
    });

    let index_json = serde_json::to_string_pretty(&index).map_err(|e| DbError::SerError(e))?;

    // Write the updated index back to disk
    fs::write(index_path, index_json)
        .await
        .map_err(|e| DbError::IoError(e))?;

    Ok(())
}
