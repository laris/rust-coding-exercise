use serde::{Serialize, Deserialize};
use derive_more::Constructor;

#[derive(Debug, Clone, Deserialize, Serialize, Constructor)]
pub struct Hits(u64);

impl Hits {
//    pub fn new(data: u64) -> Self {
//        Self(data)
//    }
    pub fn into_inner(self) -> u64 {
        self.0
    }

}