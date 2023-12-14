use crate::domain::clip::ClipError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use derive_more::From;
use rand::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, From)]
pub struct ShortCode(String);

impl ShortCode {
    pub fn new() -> Self {
        let allowed_chars = [
            'a', 'b', 'c', 'd', '1', '2', '3', '4'
        ];
        let mut rng = thread_rng();
        let mut shortcode = String::with_capacity(10);
        for _ in 0..10 {
            shortcode.push(
                *allowed_chars
                .choose(&mut rng)
                .expect("Sampling array should have values")
            );
        }
        Self(shortcode)
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Default for ShortCode {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ShortCode> for String {
    fn from(shortcode: ShortCode) -> Self {
        shortcode.0
    }
}

impl From<&str> for ShortCode {
    fn from(shortcode: &str) -> Self {
        ShortCode(shortcode.to_owned())
    }
}

impl FromStr for ShortCode {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}