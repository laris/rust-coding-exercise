use std::io::{self, BufRead};

type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;
/// Read integers from a text file.
/// The file should have one number on each line.
//fn read_numbers(file: &mut BufRead) -> Result<Vec<i64>, io::Error> {
fn read_numbers(file: &mut BufRead) -> GenResult<Vec<i64>> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;    // reading lines can fail
        numbers.push(line.parse()?); // parsing integers can fail
    }
    Ok(numbers)
}

let io_error = io::Error::new(          // make our own io::Error
    io::ErrorKind::Other, "timeout");
return Err(GenError::from(io_error)); // manually convert to GenError

loop {
    match compile_project() {
        Ok(())      => return Ok(()),
        Err(err)    => {
            if let Some(mse) = err.downcast_ref::<MissingSemicolonError>() {
                insert_semicolon_in_source_code(mse.file(), mse.line())?;
                continue; // try again!
            }
            return Err(err);
        }
    }
}