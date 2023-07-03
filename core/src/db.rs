use std::path::Path;

use sqlite::Connection;

use crate::{
    schema::v1::{self as schema, *},
    Error, Result, WithId,
};

pub struct Database {
    conn: Connection,
}

macro_rules! create_methods_for {
    ($tyname:ty, $insert_fn_name:ident, $update_name:ident, $query_description_name:ident, $query_name_name: ident) => {
        pub fn $insert_fn_name(&mut self, x: &$tyname) -> Result<()> {
            schema::$insert_fn_name(&mut self.conn, x)
        }

        pub fn $update_name(&mut self, x: &WithId<$tyname>) -> Result<()> {
            schema::$update_name(&mut self.conn, x)
        }

        pub fn $query_description_name(&mut self, query: &str) -> Result<Vec<$tyname>> {
            schema::$query_description_name(&mut self.conn, query)
        }

        pub fn $query_name_name(&mut self, query: &str) -> Result<Vec<$tyname>> {
            schema::$query_name_name(&mut self.conn, query)
        }
    };
}

impl Database {
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Self> {
        Ok(Self {
            conn: sqlite::open(path)?,
        })
    }

    pub fn create_new<T: AsRef<Path>>(path: T) -> Result<Self> {
        let path: &Path = path.as_ref();
        if path.exists() {
            Err(Error::FileExists(path.display().to_string()))
        } else {
            let mut conn = sqlite::open(path)?;
            schema::create(&mut conn)?;
            Ok(Self { conn })
        }
    }

    pub fn insert_tag(&mut self, t: &Tag) -> Result<()> {
        schema::insert_tag(&mut self.conn, t)
    }

    pub fn query_tag_name(&mut self, t: &Tag) -> Result<()> {
        schema::query_tag_name(&mut self.conn, t)
    }

    create_methods_for! { Subject, insert_subject, update_subject, query_subject_description, query_subject_name }
    create_methods_for! { Place, insert_place, update_place, query_place_description, query_place_name }
    create_methods_for! { Event, insert_event, update_event, query_event_description, query_event_name }
    create_methods_for! { Group, insert_group, update_group, query_group_description, query_group_name }
}
