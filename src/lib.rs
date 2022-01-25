use std::convert::TryInto;
use std::fmt::format;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use sled::IVec;
use tokio::fs;

pub struct Config {}

const NEXT_ID: &str = "next_id";

pub struct Index {
    db: sled::Db,
    root: String,

    next_id: u64,
}

impl Index {
    fn new(db: sled::Db, root: String) -> Self {
        let mut idx = Index {
            db,
            root,
            next_id: 0,
        };

        idx.insert_item(&Item {
            id: 2, // id should start from 2.
            parent: 2,
            name: idx.root.clone(), // Root's name is the full path.

            size: 0,
        });

        idx
    }

    /// Fetch an item from db via id.
    fn get_item(&self, id: u64) -> Result<Item> {
        let item = self.db.get(ItemKey::from(id).to_string())?;
        if item.is_none() {
            return Err(anyhow!("Item not found."));
        }

        Ok(Item::parse(item.unwrap().as_ref()))
    }

    fn get_next_id(&self) -> Result<Option<u64>> {
        let id = self.db.get(NEXT_ID)?;

        return if id.is_none() {
            Ok(None)
        } else {
            let mut buf = [0; 8];
            buf.copy_from_slice(id.unwrap().as_ref());
            Ok(Some(u64::from_le_bytes(buf)))
        };
    }

    fn set_next_id(&self, id: u64) -> Result<()> {
        self.db.insert(NEXT_ID, id.to_le_bytes().as_slice())?;
        Ok(())
    }

    /// Insert an item into db: "id -> item".
    fn insert_item(&self, item: &Item) {
        self.db
            .insert(item.as_key().to_string(), item.format())
            .expect("insert item into index");
    }

    /// Insert an item as child: "parent_id/name -> item".
    fn insert_child(&self, item: &Item) {
        self.db
            .insert(item.as_parent_key().to_string(), item.as_key().format())
            .expect("insert child into index");
    }

    /// Get a item's full path.
    fn get_full_path(&self, id: u64) -> Result<String> {
        let mut paths = Vec::new();
        loop {
            let item = self.get_item(id)?;
            paths.push(item.name.clone());
            if item.is_root() {
                break;
            }
        }

        Ok(paths.into_iter().rev().collect::<String>())
    }

    async fn scan(&self, parent: u64, name: &str) -> Result<()> {
        let path = format!("{}/{}", self.get_full_path(parent)?, name);
        let mut d = tokio::fs::read_dir(path).await?;

        loop {
            let entry = d.next_entry().await?;
            if entry.is_none() {
                break;
            }
            let entry = entry.unwrap();

            let item = Item {
                id: self.db.generate_id().unwrap(),
                parent,
                name: entry.file_name().to_string_lossy().to_string(),
                size: entry.metadata().await?.len(),
            };
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    id: u64,
    parent: u64,
    name: String,

    size: u64,
}

impl Item {
    fn is_root(&self) -> bool {
        self.id == self.parent
    }

    fn as_key(&self) -> ItemKey {
        ItemKey(self.id)
    }
    fn as_parent_key(&self) -> ItemParentKey {
        ItemParentKey(self.parent, self.name.clone())
    }

    fn format(&self) -> Vec<u8> {
        bincode::serialize(&self).expect("serialize item")
    }
    fn parse(value: &[u8]) -> Self {
        bincode::deserialize(&value).expect("deserialize item")
    }
}

pub struct ItemKey(u64);

impl ItemKey {
    pub fn format(&self) -> Vec<u8> {
        self.0.to_le_bytes().to_vec()
    }
    pub fn parse(value: &[u8]) -> Self {
        let mut buf = [0; 8];
        buf.copy_from_slice(value);
        ItemKey(u64::from_le_bytes(buf))
    }
}

impl ToString for ItemKey {
    fn to_string(&self) -> String {
        format!("ik:{}", self.0)
    }
}

impl From<u64> for ItemKey {
    fn from(id: u64) -> Self {
        ItemKey(id)
    }
}

pub struct ItemParentKey(u64, String);

impl ToString for ItemParentKey {
    fn to_string(&self) -> String {
        format!("ipk:{}:{}", self.0, self.1)
    }
}
