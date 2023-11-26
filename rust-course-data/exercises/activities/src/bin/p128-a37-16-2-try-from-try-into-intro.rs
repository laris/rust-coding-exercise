use std::convert::TryFrom;

#[derive(Debug)]
enum NonZeroError {
    IsZero,
}
#[derive(Debug)]
struct NonZero(i32);

impl TryFrom<i32> for NonZero {
    type Error = NonZeroError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(NonZeroError::IsZero)
        } else {
            Ok(NonZero(value))
        }
    }
}

fn main() {
    let target_value: Result<NonZero, NonZeroError> = NonZero::try_from(0);
    println!("{target_value:?}");
    let target_value: Result<NonZero, NonZeroError> = NonZero::try_from(1);
    println!("{target_value:?}");
}

fn main() {

}
