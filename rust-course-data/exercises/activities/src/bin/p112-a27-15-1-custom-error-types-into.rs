#[derive(Debug)]
enum LockError {
    MechanicalError(i32),
    NetworkError,
    NotAuthorized,
}

use std::error::Error as StdErr;
impl StdErr for LockError {}

use std::fmt;
impl fmt::Display for LockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MechanicalError(code) => write!(f, "Mechanical Error: {}", code),
            Self::NetworkError => write!(f, "Network Error"),
            Self::NotAuthorized => write!(f, "Not Authorized"),
        }
    }
}

use thiserror::Error as ThisErr;
#[derive(Debug, ThisErr)]
enum LockErr {
    #[error("Mechanical Error: {0}")]
    MechanicalError(i32),
    #[error("Network Error: {code}")]
    NetworkError{ code: i32 },
    #[error("Not Authorized: {0}, {1}")]
    NotAuthorized(u8, u64),
    #[error("Nested NetworkError {0}")]
    NestedNetworkError(SubNetworkError),
}

#[derive(Debug, ThisErr)]
enum SubNetworkError {
    #[error("Connection timed out: {0}{1}")]
    TimeOut(u8, String),
    #[error("Unreachable")]
    Unreachable,
}

fn lock_door() -> Result<(), LockErr> {
    Err(LockErr::NetworkError{ code: 0xff })
}
fn main() {
    println!("{:?}", lock_door());

    let timeout = SubNetworkError::TimeOut(255, "ms".into());
    let err = LockErr::NestedNetworkError(timeout);
    println!("{err:?}");
}
