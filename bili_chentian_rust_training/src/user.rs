use std::fs::File;
use serde::{Serialize, Deserialize };
use serde_json::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Gender {
    Unspecified = 0,
    Male = 1,
    Female = 2,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ConnectionState {
    Init,
    SyncReceived(HalfOpen),
    SyncAckSent(HalfOpen),
    AckReceived(FullSession),
}

pub struct HalfOpen {}
pub struct FullSession {}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    pub name: String,
    age: u8,
    pub(crate) gender: Gender,
}

impl User {
    pub fn new(name: String, age: u8, gender: Gender) -> Self {
        Self { name, age, gender }
    }

    pub fn persist(&self, filename: &str) -> Result<usize, Box<dyn Error>> {
        let mut file = File::create(filename)?;
        let user = User::default();
        let data = serde_json::to_string(&user)?;
        file.write_all(data.as_byte())?;


        Ok(data.len())
    }

    pub fn load(filename: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(filename)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let user = serde_json::from_str(&data)?;
        Ok(user)
    }
}

impl Default for User {
    fn default() -> Self {
        User::new("Unknown User".into(), 0, Gender::Unspecified)
    }
}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let user = User::default();
        let path = "/tmp/user1";
        user.persist(path).unwrap();
        let user1 = User::load(path).unwrap();
        assert_eq!(user, user1);
    }
}