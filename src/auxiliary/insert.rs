use anyhow::anyhow;

use crate::{
    access::{read_entry, read_index, read_table_mf, write_entry, write_index, write_table_mf},
    auxiliary::Error,
    data::TbEntry,
    database::Database,
    schema::{Error as SchemaError, KeyType},
    variant::{KeyVariant, Variant, VariantType},
};

pub fn insert_entry(
    db: impl AsRef<Database>,
    table: impl AsRef<str>,
    entry: impl Into<TbEntry>,
) -> Result<(), Error> {
    let db = db.as_ref();
    let table = table.as_ref();
    let entry = entry.into();

    let schema = db.schema();
    let tables = schema.tables();

    let table = tables
        .get(table)
        .ok_or(Error::from(SchemaError::NoSuchTable(table.into())))?;

    let table_name = table.name();

    let fields = table.fields();

    let p_key_field_name = table.primary_key();

    let p_key = entry
        .fields()
        .get(table.primary_key())
        .ok_or(Error::from(anyhow!(
            "primary key field must have a value: {}",
            p_key_field_name
        )))?;

    let p_key = <Variant as Into<Option<KeyVariant>>>::into(p_key.into()).ok_or(Error::from(
        SchemaError::from(anyhow!(
            "primary key field does not have key-compatible type: {}",
            p_key_field_name
        )),
    ))?;

    for (field_name, value) in entry.fields().iter() {
        let field = fields
            .get(field_name)
            .ok_or(Error::from(SchemaError::NoSuchField {
                table: table_name.into(),
                field: field_name.into(),
            }))?;

        let field_name = field.name();

        let p_key = &p_key;

        if field.value_type() != value.r#type() {
            return Err(SchemaError::TypeMismatch {
                expected: field.value_type(),
                given: value.r#type(),
            }
            .into());
        }

        if !matches!(
            field.key_type(),
            KeyType::NormalKey | KeyType::UniqueKey | KeyType::PrimaryKey
        ) || field_name == p_key_field_name
        {
            continue;
        }

        let value = value.to_owned().into_key().ok_or(Error::from(anyhow!(
            "key field does not have key-compatible type: {}",
            field_name
        )))?;

        let mut index = read_index(db, table_name, field_name)?;

        // Remove old entry
        index.entries_mut().retain(|(_, pk)| *pk != *p_key);

        if matches!(field.key_type(), KeyType::UniqueKey | KeyType::PrimaryKey) {
            // Remove old entry also
            index.entries_mut().retain(|(vk, _)| *vk != value);
        }

        index.entries_mut().insert((value, p_key.into()));

        write_index(db, table_name, field_name, index)?;
    }

    // Read entry
    let mut old_entry = read_entry(db, table_name, &p_key)?;

    // Update entry
    old_entry.fields_mut().extend(entry.get_fields());

    // Make sure all fields are present
    for (_, f) in fields.iter() {
        let et_fields = old_entry.fields_mut();

        let f_name = f.name();

        if et_fields.contains_key(f_name) {
            continue;
        }

        match f.value_type() {
            // Create missing blobs rather than erroring out
            VariantType::Blob => {
                et_fields.insert(f_name.into(), Variant::Blob);

                // TODO: Create blob file using `access` module
            }

            // Create missing option fields as `None` rather than erroring out
            VariantType::Option => {
                et_fields.insert(f_name.into(), Variant::Option(None));
            }

            _ => {
                return Err(anyhow::anyhow!("field has no value: {}", f.name()).into());
            }
        }
    }

    // Write entry
    write_entry(db, table_name, p_key.clone(), old_entry)?;

    // Update table manifest
    let mut table_mf = read_table_mf(db, table_name)?;
    table_mf.entries_mut().insert(p_key);
    write_table_mf(db, table_name, table_mf)?;

    Ok(())
}
