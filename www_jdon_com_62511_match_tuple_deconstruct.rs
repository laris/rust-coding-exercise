fn main() {

    match (|| Ok(fn1()?, fn2()?, fn3()?))() {
        Ok((r1, r2, r3)) => handleResponse(r1, r2, r3),
        Err(e) => handleErr(e),
    }
}