use serde::{Deserialize, Serialize};
use thiserror::Error;
pub mod field;

/// The possible errors that can occur when building a [`Clip`]
#[derive(Debug, Error)]
pub enum ClipError {
    /// Password does not meet complexity requirements.
    #[error("invalid password: {0}")]
    InvalidPassword(String),

    /// Clip title has unwanted words/data.
    #[error("invalid title: {0}")]
    InvalidTitle(String),

    /// Content was not provided.
    #[error("empty content")]
    EmptyContent,

    /// Date is invalid: invalid day of the month, too far in the past, etc.
    #[error("invalid date: {0}")]
    InvalidDate(String),

    /// Date failed to parse.
    #[error("date parse error: {0}")]
    DateParse(#[from] chrono::ParseError),

    /// [crate::data::DbId] failed to parse.
    #[error("id parse error: {0}")]
    Id(#[from] uuid::Error),

    /// Number of hits is negative or not a number.
    #[error("hits parse error: {0}")]
    Hits(#[from] std::num::TryFromIntError),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Clip {
    pub clip_id: field::ClipId,
    pub shortcode: field::ShortCode,
    pub content: field::Content,
    pub title: field::Title,
    pub posted: field::Posted,
    pub expires: field::Expires,
    pub password: field::Password,
    pub hits: field::Hits
}
