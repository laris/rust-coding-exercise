use chrono::{DateTime, Duration, Utc};
use thiserror::Error;

struct SubwayPass {
    id: usize,
    funds: isize,
    expires: DateTime<Utc>,
}

enum PassError {
    PassExpired,
    InsufficientFunds(isize),
    ReadError(String),
}


