pub mod field;
use thiserror::Error;
use chrono::{self};
use serde::{Deserialize, Serialize};

#[derive(Debug,Error)]
pub enum ClipError {
    #[error("id parse error: {0}")]
    Id(#[from] uuid::Error),
    #[error("invalid password: {0}")]
    InvalidPassword(String),
    #[error("invalid title: {0}")]
    InvalidTitle(String),
    #[error("empty content")]
    EmptyContent,
    #[error("invalid date: {0}")]
    InvalidDate(String),
    #[error("date parse error: {0}")]
    DateParse(#[from] chrono::ParseError),
    #[error("hits parse error: {0}")]
    Hits(#[from] std::num::TryFromIntError),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    #[serde(skip)]
    pub clip_id: field::ClipId,
    pub shortcode: field::ShortCode,
    pub content: field::Content,
    pub title: field::Title,
    pub posted: field::Posted,
    pub expires: field::Expires,
    pub password: field::Password,
    pub hits: field::Hits,
}