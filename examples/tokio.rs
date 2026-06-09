use nahsql::{
    database::Database,
    schema::{Schema, SchemaField, SchemaTable},
    value::ValueType,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let schema = Schema::new(
        0,
        vec![SchemaTable::new(
            "users",
            "id",
            vec![
                SchemaField::new("id", true, ValueType::U64),
                SchemaField::new("name", true, ValueType::String),
                SchemaField::new("passphrase", false, ValueType::String),
            ],
        )],
    );

    let _db = Database::open_async("./database", schema).await?;

    Ok(())
}
