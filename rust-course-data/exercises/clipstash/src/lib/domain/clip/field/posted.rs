//use crate::domain::clip::ClipError;
use crate::domain::time::Time;
use serde::{Serialize, Deserialize};
use derive_more::Constructor;

#[derive(Clone, Debug, Constructor, Deserialize, Serialize)]
pub struct Posted(Time);

impl Posted {
    pub fn into_inner(self) -> Time {
        self.0
    }
}