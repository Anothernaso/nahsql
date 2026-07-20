use nahsql::{auxiliary::*, database::*, schema::*, value::*};

fn main() -> anyhow::Result<()> {
    let schema = Schema::new(
        0,
        vec![
            SchemaTable::new(
                "users",
                vec![
                    SchemaField::new("id", KeyType::PrimaryKey, ValueType::U64),
                    SchemaField::new("username", KeyType::NormalKey, ValueType::String),
                    SchemaField::new("email", KeyType::UniqueKey, ValueType::String),
                    SchemaField::new("passphrase", KeyType::NonKey, ValueType::String),
                ],
            ),
            SchemaTable::new(
                "posts",
                vec![
                    SchemaField::new("id", KeyType::PrimaryKey, ValueType::U64),
                    SchemaField::new("user_id", KeyType::NormalKey, ValueType::U64),
                    SchemaField::new("title", KeyType::NonKey, ValueType::String),
                    SchemaField::new("content", KeyType::NonKey, ValueType::String),
                ],
            ),
            SchemaTable::new(
                "likes",
                vec![
                    SchemaField::new("id", KeyType::PrimaryKey, ValueType::U64),
                    SchemaField::new("user_id", KeyType::NormalKey, ValueType::U64),
                    SchemaField::new("post_id", KeyType::NormalKey, ValueType::U64),
                ],
            ),
            SchemaTable::new(
                "followings",
                vec![
                    SchemaField::new("id", KeyType::PrimaryKey, ValueType::U64),
                    SchemaField::new("user_id", KeyType::NormalKey, ValueType::U64),
                    SchemaField::new("following_id", KeyType::NormalKey, ValueType::U64),
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
