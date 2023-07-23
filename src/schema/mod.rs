use std::ops::{Deref, DerefMut};
use std::result::Result as StdResult;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("SQLite error: {0:?}")]
    SQLite(#[from] sqlite::Error),

    #[error("The file can't be created, it already exists: {0}")]
    FileExists(String),
}

pub type Result<T> = StdResult<T, Error>;

#[derive(Clone)]
pub struct WithId<T> {
    pub t: T,
    pub id: i64,
}

impl<T> Deref for WithId<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.t
    }
}

impl<T> DerefMut for WithId<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.t
    }
}

pub mod v1;
