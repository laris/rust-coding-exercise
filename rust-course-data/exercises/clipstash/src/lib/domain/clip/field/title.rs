use crate::domain::clip::ClipError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Title(Option<String>);

impl Title {
    pub fn new<T: Into<Option<String>>>(title: T) -> Result<Self, ClipError> {
        let title: Option<String> = title.into();
        match title {
            Some(title) => {
                if !title.trim().is_empty() {
                    Ok(Self(Some(title)))
                } else {
                    Ok(Self(None))
                }
            },
            None => Ok(Self(None))
        }
    }
    pub fn into_inner(self) -> Option<String> {
        self.0
    }
}

impl Default for Title {
    fn default() -> Self {
        //Self::new(None)
        Self(None)
    }
}

impl FromStr for Title {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}
