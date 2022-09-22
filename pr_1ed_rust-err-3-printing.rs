println!("Error querying the weather: {}", err);

// result of `println!("error: {}",err);`
error: failed to lookup address information: No address associated with hostname
// result of `println!("error: {:?}", err);`
error: Error { repr: Custom(Custom { kind: Other, error: StringError( "failed to lookup address information: No address associated with hostname") }) }

use std::error::Error;
use std::io::{Write, stderr};

/// Dump a error message to `stderr`
/// If another error happens while building the error message or
/// writing to `stderr`, it is ignored
fn print_error(mut err: &Error) {
    let _ = writeln!(stderr(), "error: {}", err); 
    while let Some(cause) = err.cause() {
        let _ = writeln!(stderr(), "caused by: {}", cause);
        err = cause;
    }
}

