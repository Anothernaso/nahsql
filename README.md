# NahSQL
> **Disclaimer**: NahSQL is in early stages of development and is not currently functional.

NahSQL is a simple and purpose-built database inspired by SQLite
but differs in the way it is queried using a Rust API rather than SQL.

# Examples

Synchronous, without `tokio`:
```rust
use nahsql::{database::Database, schema::SchemaBuilder, value::ValueType};

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
```

Asynchronous, with `tokio`:
```rust
use nahsql::{database::Database, schema::SchemaBuilder, value::ValueType};

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
```
