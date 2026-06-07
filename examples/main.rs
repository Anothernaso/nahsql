#[cfg(all(feature = "std", feature = "tokio"))]
use std::process::exit;

#[cfg(all(feature = "std", feature = "tokio"))]
use colored::Colorize;

#[cfg(all(feature = "std", feature = "tokio"))]
fn main() {
    eprintln!(
        "{}{}",
        "Error".bright_red(),
        ": You may not enable both the `std` and the `tokio` features in this example"
    );

    exit(1);
}

#[cfg(all(
    any(feature = "std", feature = "tokio"),
    not(all(feature = "std", feature = "tokio"))
))]
use nahsql::{database::Database, schema::SchemaBuilder, value::ValueType};

#[cfg(all(feature = "std", not(feature = "tokio")))]
fn main() -> anyhow::Result<()> {
    let schema = SchemaBuilder::new(|schema| {
        Ok(schema.table("users", |table| {
            Ok(table
                .field("id", |field| Ok(field.r#type(ValueType::String).key()))?
                .field("name", |field| Ok(field.r#type(ValueType::String).key()))?
                .field("passphrase", |field| Ok(field.r#type(ValueType::String)))?
                .primary_key("id"))
        })?)
    })?;

    let db = Database::open_sync("./database", schema)?;

    Ok(())
}

#[cfg(all(feature = "tokio", not(feature = "std")))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let schema = SchemaBuilder::new(|schema| {
        Ok(schema.table("users", |table| {
            Ok(table
                .field("id", |field| Ok(field.r#type(ValueType::String).key()))?
                .field("name", |field| Ok(field.r#type(ValueType::String).key()))?
                .field("passphrase", |field| Ok(field.r#type(ValueType::String)))?
                .primary_key("id"))
        })?)
    })?;

    let db = Database::open_async("./database", schema).await?;

    Ok(())
}
