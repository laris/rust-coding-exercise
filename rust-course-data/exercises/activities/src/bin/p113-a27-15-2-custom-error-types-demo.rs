use chrono::{DateTime, Duration, Utc};
use thiserror::Error;

struct SubwayPass {
    id: usize,
    funds: isize,
    expires: DateTime<Utc>,
}

#[derive(Debug, Error)]
enum PassError {
    #[error("expired pass")]
    PassExpired,
    #[error("insufficient funds: {0}")]
    InsufficientFunds(isize),
    #[error("pass read error: {0}")]
    ReadError(String),
}

fn swipe_card() -> Result<SubwayPass, PassError> {
    //Ok(SubwayPass { id: 0, funds: 200, expires: Utc::now() + Duration::weeks(52), })
    Ok(SubwayPass { id: 0, funds: 200, expires: Utc::now() - Duration::weeks(52), })
    //Err(PassError::ReadError("Magstrip failed to read".into()))
}

fn use_pass(pass: &mut SubwayPass, cost: isize) -> Result<(), PassError> {
    if Utc::now() > pass.expires {
        Err(PassError::PassExpired)
    } else {
        if pass.funds - cost < 0 {
            Err(PassError::InsufficientFunds(pass.funds))
        } else {
            pass.funds = pass.funds - cost;
            Ok(())
        }
    }
}

fn main() {
    let pass_status = swipe_card().and_then(|mut pass| use_pass(&mut pass, 3));
    match pass_status {
        Ok(_) => println!("ok to board"),
        Err(e) => println!("Error: {e:?}"),
    }
}
