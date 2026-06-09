# NahSQL
> **Disclaimer**: NahSQL is in early stages of development and is not currently functional.

NahSQL is a simple and purpose-built database inspired by SQLite
but differs in the way it is queried using a Rust API rather than SQL.

# Examples

Synchronous, without `tokio`:
```rust
use nahsql::{
    database::Database,
    schema::{Schema, SchemaField, SchemaTable},
    value::ValueType,
};

fn main() -> anyhow::Result<()> {
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

    let _db = Database::open_sync("./database", schema)?;

    Ok(())
}
```

Asynchronous, with `tokio`:
```rust
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
```
