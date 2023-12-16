use crate::domain::clip::ClipError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use rocket::form::{self, FromFormField, ValueField};
use crate::domain::clip::field::Expires;

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

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Title{
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        if field.value.trim().is_empty() {
            Ok(Self(None))
        } else {
            Ok(Self::from_str(field.value).unwrap_or_default())
        }
    }
}
