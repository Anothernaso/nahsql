use crate::{
    access::{read_entry, read_index, read_table_mf, remove_blob, write_index, write_table_mf},
    auxiliary::Error,
    database::Database,
    schema::Error as SchemaError,
    variant::{KeyVariant, VariantType},
};

/// Removes an entry from the given table
pub fn remove_entry(
    db: impl AsRef<Database>,
    table: impl AsRef<str>,
    primary_key: impl Into<KeyVariant>,
) -> Result<(), Error> {
    let db = db.as_ref();
    let table = table.as_ref();
    let primary_key = primary_key.into();

    let schema = db.schema();
    let tables = schema.tables();

    let table = tables
        .get(table)
        .ok_or(Error::from(SchemaError::NoSuchTable(table.into())))?;
    let table_name = table.name();

    //
    // Removal
    //

    let entry = read_entry(db, table_name, primary_key.clone())?;
    let entry_fields = entry.fields();

    for (field_name, field) in entry_fields {
        // Remove blobs
        if field.r#type() == VariantType::Blob {
            remove_blob(db, table_name, primary_key.clone(), field_name)?;
        }

        // Remove from indices
        let mut index = read_index(db, table_name, field_name)?;
        index
            .entries_mut()
            .retain(|(_, p_key)| *p_key != primary_key);
        write_index(db, table_name, field_name, index)?;
    }

    // Remove entry
    remove_entry(db, table_name, primary_key.clone())?;

    // Unregister the entry from the table manifest
    let mut table_mf = read_table_mf(db, table_name)?;
    table_mf.entries_mut().remove(&primary_key);
    write_table_mf(db, table_name, table_mf)?;

    Ok(())
}
