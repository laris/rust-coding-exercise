struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

fn main() {
    let response = Survey {
        //q1: Some(12),
        q1: None,
        q2: Some(true),
        q3: Some("A".into()),
    };
    match response.q1 {
        Some(ans) => println!("q1: {:?}", ans),
        _ => println!("q1, no response"),
    }
    match response.q2 {
        Some(ans) => println!("q2: {:?}", ans),
        _ => println!("q2, no response"),
    }
    match response.q3 {
        Some(ans) => println!("q3: {:?}", ans),
        _ => println!("q3, no response"),
    }

}
