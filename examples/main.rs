use std::collections::HashMap;

use nahsql::{auxiliary::*, data::TbEntry, database::*, schema::*, value::*};

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

    insert_entry(
        &db,
        "users",
        TbEntry::new(HashMap::from([
            ("id".into(), Value::U64(0)),
            ("username".into(), Value::String("Anatnaso".into())),
            ("email".into(), Value::String("antondev@kuthy.com".into())),
            ("passphrase".into(), Value::String("[REDACTED]".into())),
        ])),
    )?;

    insert_entry(
        &db,
        "users",
        TbEntry::new(HashMap::from([
            ("id".into(), Value::U64(1)),
            ("username".into(), Value::String("Greger".into())),
            ("email".into(), Value::String("greger@example.org".into())),
            (
                "passphrase".into(),
                Value::String("ilovemustard123!".into()),
            ),
        ])),
    )?;

    insert_entry(
        &db,
        "posts",
        TbEntry::new(HashMap::from([
            ("id".into(), Value::U64(0)),
            ("title".into(), Value::String("How to Eat Mustard".into())),
            (
                "content".into(),
                Value::String(
                    r#"
1. Acquire mustard. Resist the temptation to salute it.
2. Open the container carefully. Mustard has a surprising talent for appearing where you least expect it.
3. Locate food. A hot dog, pretzel, sandwich, or spoon if you're feeling unusually adventurous.
4. Apply a sensible amount. If it looks like a yellow avalanche, you've gone too far. Or not far enough. Who can say?
5. Take a bite.
6. Nod thoughtfully as though you're evaluating a vintage condiment.
7. If your eyes water a little, declare, "Ah yes, the flavor is working."
8. Repeat until the food, or the mustard, runs out.
9. Congratulate yourself on another successful encounter with one of humanity's boldest yellow inventions.
"#
                    .into(),
                ),
            ),
        ])),
    )?;

    insert_entry(
        &db,
        "likes",
        TbEntry::new(HashMap::from([
            ("id".into(), Value::U64(0)),
            ("user_id".into(), Value::U64(0)),
            ("post_id".into(), Value::U64(0)),
        ])),
    )?;

    insert_entry(
        &db,
        "followings",
        TbEntry::new(HashMap::from([
            ("id".into(), Value::U64(0)),
            ("user_id".into(), Value::U64(0)),
            ("following_id".into(), Value::U64(1)),
        ])),
    )?;

    Ok(())
}
