use nahsql::{auxiliary::*, database::*, schema::*, value::*};

fn main() -> anyhow::Result<()> {
    let schema = Schema::new(
        0,
        vec![
            SchemaTable::new(
                "users",
                "id",
                vec![
                    SchemaField::new("id", true, ValueType::U64),
                    SchemaField::new("username", true, ValueType::String),
                    SchemaField::new("email", false, ValueType::String),
                    SchemaField::new("passphrase", false, ValueType::String),
                    SchemaField::new("followers", false, ValueType::U64),
                ],
            ),
            SchemaTable::new(
                "posts",
                "id",
                vec![
                    SchemaField::new("id", true, ValueType::U64),
                    SchemaField::new("user_id", false, ValueType::U64),
                    SchemaField::new("title", true, ValueType::String),
                    SchemaField::new("content", false, ValueType::String),
                ],
            ),
            SchemaTable::new(
                "likes",
                "id",
                vec![
                    SchemaField::new("id", true, ValueType::U64),
                    SchemaField::new("user_id", true, ValueType::U64),
                    SchemaField::new("post_id", true, ValueType::U64),
                ],
            ),
        ],
    );

    let db = Database::open("./database", schema)?;

    for (table_name, _) in db.schema().tables() {
        create_indices(&db, table_name)?;
    }

    Ok(())
}
