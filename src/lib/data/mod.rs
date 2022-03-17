use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::str::FromStr;

#[derive(Clone, Debug, From, Display, Deserialize, Serialize)]
pub struct DbId(Uuid);

impl DbId {
    /// Create a new database ID.
    pub fn new() -> DbId {
        Uuid::new_v4().into()
    }

    /// Create an empty database ID.
    ///
    /// This database ID is always the same. It can be used to obscure an
    /// actual ID when working with clients.
    pub fn nil() -> DbId {
        Self(Uuid::nil())
    }
}

impl From<DbId> for String {
    fn from(id: DbId) -> Self {
        format!("{}", id.0)
    }
}

/// The default behavior is to create a [`DbId`]
impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}


impl FromStr for DbId {
    type Err = uuid::Error;
    
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(DbId(Uuid::parse_str(id)?))
    }
}
