//# thiserror = "*"

#[derive(Debug)]
struct Uppercase(String);

impl From<String> for Uppercase {
    fn from(data: String) -> Self {
       Self(data.to_uppercase()) 
    }
}

impl From<&str> for Uppercase {
    fn from(data: &str) -> Self {
        Self(data.to_uppercase())
    }
}
// demo 2
#[derive(Debug, Copy, Clone)]
enum KeyPress {
    Down,
    Up,
}

#[derive(Debug, Copy, Clone)]
struct KeyEvent {
    keycode: u16,
    state: KeyPress,
}

#[derive(Debug)]
enum InputEvent {
    Key(u16, KeyPress),
    Mouse,
}

impl From<KeyEvent> for InputEvent {
    fn from(ev: KeyEvent) -> Self {
        Self::Key(ev.keycode, ev.state)
    }
}
//demo 3
use thiserror::Error;
#[derive(Debug, Error)]
enum NetworkError {
    #[error("connection time out")]
    Timeout,
}
#[derive(Debug, Error)]
enum DatabaseError {
    #[error("error querying database")]
    QueryFailure,
}
#[derive(Debug, Error)]
enum ApiError {
    #[error("network error: {0}")]
    Network(NetworkError),
    //Network(#[from] NetworkError),
    #[error("database error: {0}")]
    Database(DatabaseError),
    //Database(#[from] DatabaseError),
}

impl From<NetworkError> for ApiError {
    fn from(err: NetworkError) -> Self {
        Self::Network(err)
    }
}

impl From<DatabaseError> for ApiError {
    fn from(err: DatabaseError) -> Self {
        Self::Database(err)
    }
}

fn do_stuff() -> Result<(), ApiError> {
    Err(NetworkError::Timeout)?
}

fn main() {
    let upper = Uppercase::from("lowercase");
    println!("{upper:?}");
    let upper: Uppercase = "lowercase".into();
    println!("{upper:?}");
    //demo2
    let key_ev = KeyEvent {
        keycode: 5,
        state: KeyPress::Down,
    };
    let input_ev = InputEvent::from(key_ev);
    println!("{input_ev:?}");
    let input_ev: InputEvent = key_ev.into();
    println!("{input_ev:?}");

    // demo 3
    println!("{:?}", do_stuff());
}
