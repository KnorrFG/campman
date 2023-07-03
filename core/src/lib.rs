use std::ops::Deref;
use std::result::Result as StdResult;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("SQLite error: {0:?}")]
    SQLite(#[from] sqlite::Error),

    #[error("The file can't be created, it already exists: {0}")]
    FileExists(String),
}

pub type Result<T> = StdResult<T, Error>;

pub struct WithId<T> {
    pub t: T,
    pub id: u64,
}

impl<T> Deref for WithId<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.t
    }
}

mod db;
pub use db::*;

pub mod schema;
