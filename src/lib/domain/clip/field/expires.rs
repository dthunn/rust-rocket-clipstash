// use rocket::form::{self, FromFormField, ValueField};
use crate::domain::time::Time;
use crate::domain::clip::ClipError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// The expiration date field for a [`Clip`](crate::domain::clip::Clip).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Expires(Option<Time>);

impl Expires {
    /// Create a new `Expires` field.
    pub fn new<T: Into<Option<Time>>>(expires: T) -> Self {
        Self(expires.into())
    }
    /// Return the underlying [`Option<Time>`](crate::domain::time::Time).
    pub fn into_inner(self) -> Option<Time> {
        self.0
    }
}

/// The Default implementation is no expiration date.
impl Default for Expires {
    fn default() -> Self {
        Self::new(None)
    }
}

impl FromStr for Expires {
    type Err = ClipError;
    
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.is_empty() {
            Ok(Self(None))
        } else {
            match Time::from_str(raw) {
                Ok(time) => Ok(Self::new(time)),
                Err(e) => Err(e.into())
            }
        }
    }
}