use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};

use super::schema::items;

#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct Item {
    id: i64,
    parent: i64,
    name: String,

    size: i64,
}

impl Item {
    fn is_root(&self) -> bool {
        self.id == self.parent
    }
}


#[derive(Insertable)]
#[table_name = "items"]
pub struct NewItem<'a> {
    parent: &'a i64,
    name: &'a str,
    size: &'a i64,
}

fn create_item(conn: &SqliteConnection, parent: i64, name: String, size: i64) -> Result<()> {
    let new_item = NewItem {
        parent: &parent,
        name: &name,
        size: &size,
    };

    diesel::insert_into(items::table)
        .values(&new_item)
        .execute(conn)?;

    Ok(())
}