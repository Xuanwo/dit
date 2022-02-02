use std::{env, iter};

use dotenv::dotenv;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub struct DB {
    conn: SqliteConnection,
}

impl DB {
    pub fn new(path: &str) -> DB {
        dotenv().ok();

        let conn = SqliteConnection::establish(path)
            .expect(&format!("Error connecting to {}", path));

        DB {
            conn
        }
    }
}

