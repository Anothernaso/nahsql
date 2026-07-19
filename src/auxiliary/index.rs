use anyhow::anyhow;

use crate::{
    access::{read_entry, read_index, read_table_mf, write_index},
    auxiliary::Error,
    database::Database,
    schema::Error as SchemaError,
    value::{Value, ValueKey},
};

/// Creates indices for all fields of the given table in the database.
pub fn create_indices(db: impl AsRef<Database>, table: impl AsRef<str>) -> Result<(), Error> {
    let db = db.as_ref();
    let table = table.as_ref();

    let schema = db.schema();
    let table = schema
        .tables()
        .get(table)
        .ok_or(Error::from(SchemaError::NoSuchTable(table.into())))?;

    let table_name = table.name();

    let fields = table.fields();

    let p_key_field = table.primary_key();
    let p_key_field = fields
        .get(p_key_field)
        .ok_or(Error::from(SchemaError::NoSuchField {
            table: table_name.into(),
            field: p_key_field.into(),
        }))?;

    let p_key_field_name = p_key_field.name();

    let table_mf = read_table_mf(db, table_name)?;

    // For each key field except the primary key field
    for (_, field) in fields
        .iter()
        .filter(|(_, f)| f.is_key() && f.name() != p_key_field_name)
    {
        let field_name = field.name();

        let mut index = read_index(db, table_name, field_name)?;

        // For each entry
        for p_key in table_mf.entries.iter() {
            let entry = read_entry(db, table_name, p_key.clone())?;

            let value = entry.fields.get(field_name).ok_or(anyhow!(
                "entry `{}` has missing field: {}",
                p_key,
                field_name
            ))?;

            let value = <Value as Into<Option<ValueKey>>>::into(value.clone()).ok_or(anyhow!(
                "field `{}` of entry `{}` has non-key value type: {}",
                field_name,
                p_key,
                value.r#type()
            ))?;

            index.entries.insert(value, p_key.clone());
        }

        write_index(db, table_name, field_name, index)?;
    }

    Ok(())
}
