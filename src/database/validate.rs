use crate::database::{
    Database, DbError,
    manifest::{DbManifest, DbManifestImpl},
    table::validate::DbTableValidateImpl,
};

pub trait DbValidateImpl {
    #[cfg(all(feature = "std"))]
    fn validate_sync(&self) -> Result<(), DbError>;

    #[cfg(all(feature = "tokio"))]
    fn validate_async(&self) -> impl Future<Output = Result<(), DbError>> + Send;
}

impl DbValidateImpl for Database {
    /// Makes the database on the filesystem match its schema.
    ///
    /// Returns a `DbError` if anything goes wrong.
    ///
    #[cfg(all(feature = "std"))]
    fn validate_sync(&self) -> Result<(), DbError> {
        use std::fs;

        // Create the database directory if it doesn't exist
        if !fs::exists(&self.path).map_err(|e| DbError::IoError(e))? {
            fs::create_dir_all(&self.path).map_err(|e| DbError::IoError(e))?;
        }

        let table_dir = self.table_dir();

        // Create the table directory if it doesn't exist
        if !fs::exists(&table_dir).map_err(|e| DbError::IoError(e))? {
            fs::create_dir_all(table_dir).map_err(|e| DbError::IoError(e))?;
        }

        // Create the manifest if it doesn't exist
        let mf_path = self.mf_path();
        if !fs::exists(&mf_path).map_err(|e| DbError::IoError(e))? {
            fs::write(
                mf_path,
                serde_json::to_string_pretty(&DbManifest::new(self.schema.version()))
                    .map_err(|e| DbError::SerError(e))?,
            )
            .map_err(|e| DbError::IoError(e))?;
        }

        // Validate all tables
        for (_, table) in self.schema.tables().iter() {
            self.validate_table_sync(table.name())?;
        }

        Ok(())
    }

    /// Makes the database on the filesystem match its schema.
    ///
    /// Returns a `DbError` if anything goes wrong.
    ///
    #[cfg(all(feature = "tokio"))]
    fn validate_async(&self) -> impl Future<Output = Result<(), DbError>> {
        use tokio::fs;

        async move {
            // Create the database directory if it doesn't exist
            if !fs::try_exists(&self.path)
                .await
                .map_err(|e| DbError::IoError(e))?
            {
                fs::create_dir_all(&self.path)
                    .await
                    .map_err(|e| DbError::IoError(e))?;
            }

            let table_dir = self.table_dir();

            // Create the table directory if it doesn't exist
            if !fs::try_exists(&table_dir)
                .await
                .map_err(|e| DbError::IoError(e))?
            {
                fs::create_dir_all(table_dir)
                    .await
                    .map_err(|e| DbError::IoError(e))?;
            }

            // Create the manifest if it doesn't exist
            let mf_path = self.mf_path();
            if !fs::try_exists(&mf_path)
                .await
                .map_err(|e| DbError::IoError(e))?
            {
                fs::write(
                    mf_path,
                    serde_json::to_string_pretty(&DbManifest::new(self.schema.version()))
                        .map_err(|e| DbError::SerError(e))?,
                )
                .await
                .map_err(|e| DbError::IoError(e))?;
            }

            // Validate all tables
            for (_, table) in self.schema.tables().iter() {
                self.validate_table_async(table.name()).await?;
            }

            Ok(())
        }
    }
}
