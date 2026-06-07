use crate::database::{
    Database, DbError,
    table::{
        DbTableImpl,
        entry::DbTableEntryImpl,
        index::{DbTableIndex, DbTableIndexImpl},
    },
};

pub trait DbTableValidateImpl {
    #[cfg(all(feature = "std"))]
    fn validate_table_sync(&self, table_name: impl AsRef<str>) -> Result<(), DbError>;

    #[cfg(all(feature = "tokio"))]
    fn validate_table_async(
        &self,
        table_name: impl AsRef<str> + Send,
    ) -> impl Future<Output = Result<(), DbError>> + Send;
}

impl DbTableValidateImpl for Database {
    /// Makes the table on the filesystem match the database's schema.
    ///
    /// Returns a `DbError` if anything goes wrong.
    ///
    #[cfg(all(feature = "std"))]
    fn validate_table_sync(&self, table_name: impl AsRef<str>) -> Result<(), DbError> {
        use std::fs;

        let table_name = table_name.as_ref();
        let tables = self.schema.tables();

        // Get the table in the schema, return if it doesn't exist
        let Some(table) = tables.get(table_name) else {
            return Ok(());
        };

        let table_path = self.table_path(table_name);

        // Create the table directory if it doesn't exist
        if !fs::exists(&table_path).map_err(|e| DbError::IoError(e))? {
            fs::create_dir_all(table_path).map_err(|e| DbError::IoError(e))?;
        }

        let index_dir = self.table_index_dir(table_name);

        // Create the index directory if it doesn't exist
        if !fs::exists(&index_dir).map_err(|e| DbError::IoError(e))? {
            fs::create_dir_all(index_dir).map_err(|e| DbError::IoError(e))?;
        }

        let entry_dir = self.table_entry_dir(table_name);

        // Create the entry directory if it doesn't exist
        if !fs::exists(&entry_dir).map_err(|e| DbError::IoError(e))? {
            fs::create_dir_all(entry_dir).map_err(|e| DbError::IoError(e))?;
        }

        let fields = table.fields();
        let pkey_field = fields.get(table.primary_key()).ok_or_else(|| {
            DbError::SchemaError(
                "primary key does not have a respective field in schema".to_owned(),
            )
        })?;

        // Validate all indices of the table.
        //
        // Each field that is marked as a key should have an index,
        // except the primary key.
        //
        for (_, field) in fields
            .iter()
            .filter(|(_, f)| f.is_key() && pkey_field.name() != f.name())
        {
            let field_name = field.name();
            let index_path = self.table_index_path(table_name, field_name);

            let mut index: DbTableIndex;

            // Create the index if it doesn't exist
            if !fs::exists(&index_path).map_err(|e| DbError::IoError(e))? {
                index = DbTableIndex::new();
            }
            // Read the index if it already exists
            else {
                let index_json =
                    fs::read_to_string(&index_path).map_err(|e| DbError::IoError(e))?;

                index = serde_json::from_str(&index_json).map_err(|e| DbError::SerError(e))?;
            }

            // Remove all entries from the index that don't match the types in the schema
            index.entries_mut().retain(|fkey, pkey| {
                fkey.r#type() == field.r#type() && pkey.r#type() == pkey_field.r#type()
            });

            let index_json =
                serde_json::to_string_pretty(&index).map_err(|e| DbError::SerError(e))?;

            // Write the updated index back to disk
            fs::write(index_path, index_json).map_err(|e| DbError::IoError(e))?;
        }

        Ok(())
    }

    /// Makes the table on the filesystem match the database's schema.
    ///
    /// Returns a `DbError` if anything goes wrong.
    ///
    #[cfg(all(feature = "tokio"))]
    fn validate_table_async(
        &self,
        table_name: impl AsRef<str> + Send,
    ) -> impl Future<Output = Result<(), DbError>> + Send {
        use tokio::fs;

        async move {
            let table_name = table_name.as_ref();
            let tables = self.schema.tables();

            // Get the table in the schema, return if it doesn't exist
            let Some(table) = tables.get(table_name) else {
                return Ok(());
            };

            let table_path = self.table_path(table_name);

            // Create the table directory if it doesn't exist
            if !fs::try_exists(&table_path)
                .await
                .map_err(|e| DbError::IoError(e))?
            {
                fs::create_dir_all(table_path)
                    .await
                    .map_err(|e| DbError::IoError(e))?;
            }

            let index_dir = self.table_index_dir(table_name);

            // Create the index directory if it doesn't exist
            if !fs::try_exists(&index_dir)
                .await
                .map_err(|e| DbError::IoError(e))?
            {
                fs::create_dir_all(index_dir)
                    .await
                    .map_err(|e| DbError::IoError(e))?;
            }

            let entry_dir = self.table_entry_dir(table_name);

            // Create the entry directory if it doesn't exist
            if !fs::try_exists(&entry_dir)
                .await
                .map_err(|e| DbError::IoError(e))?
            {
                fs::create_dir_all(entry_dir)
                    .await
                    .map_err(|e| DbError::IoError(e))?;
            }

            let fields = table.fields();
            let pkey_field = fields.get(table.primary_key()).ok_or_else(|| {
                DbError::SchemaError(
                    "primary key does not have a respective field in schema".to_owned(),
                )
            })?;

            // Validate all indices of the table.
            //
            // Each field that is marked as a key should have an index,
            // except the primary key.
            //
            for (_, field) in fields
                .iter()
                .filter(|(_, f)| f.is_key() && pkey_field.name() != f.name())
            {
                let field_name = field.name();
                let index_path = self.table_index_path(table_name, field_name);

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

                let index_json =
                    serde_json::to_string_pretty(&index).map_err(|e| DbError::SerError(e))?;

                // Write the updated index back to disk
                fs::write(index_path, index_json)
                    .await
                    .map_err(|e| DbError::IoError(e))?;
            }

            Ok(())
        }
    }
}
