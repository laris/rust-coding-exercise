fn main() {
    let owned = String::from("slice");
    let owned: String = "slice".into();

    fn to_owned(slice: &str) -> String {
        slice.into()
    }

    let owned = to_owned("slice");
    println!("{owned:?}");

    // 2nd demo
#[derive(Debug)]
    enum Status {
        Broken(u8),
        Working,
    }
    impl From<u8> for Status {
        fn from(code: u8) -> Self {
            match code {
                0 => Status::Working,
                1 => Status::Broken(code),
                code @ _ => Status::Broken(code),
            }
        }
    }

    let status: Status = 0.into();
    println!("{status:?}");
    let status: Status = 1.into();
    println!("{status:?}");
    let status: Status = 100.into();
    println!("{status:?}");

    fn legacy_interface(num: u8) -> u8 { num }
    let status = Status::from(legacy_interface(1));
    println!("{status:?}");

    // demo 3
    struct Job;
    #[derive(Debug)]
    enum JobError {
        Expired,
        Missing,
        Other(u8),
    }
    impl From<u8> for JobError {
        fn from(code: u8) -> Self {
            match code {
                1 => Self::Expired,
                2 => Self::Missing,
                c => Self::Other(c),
            }
        }
    }

    fn execute_job(job: Job) -> Result<(), JobError> {
        Err(2)?
    }
    println!("{:?}", execute_job(Job));
}
